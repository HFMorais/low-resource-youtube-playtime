// Crate Dependencies ---------------------------------------------------------
// ----------------------------------------------------------------------------
extern crate cursive;
extern crate cursive_table_view;
extern crate rand;

// STD Dependencies -----------------------------------------------------------
// ----------------------------------------------------------------------------
use std::cmp::Ordering;

// External Dependencies ------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, TextView, CircularFocus, EditView, OnEventView};
use cursive::Cursive;
use cursive::event::{Event, Key};

// Modules --------------------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive_table_view::{TableView, TableViewItem};

use crate::ui::videos_window;

use crate::data_structures::Channel;
use crate::{channel_parser, database_handler};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Name,
    Url,
}

impl BasicColumn {
    fn as_str(&self) -> &str {
        match *self {
            BasicColumn::Name => "Name",
            BasicColumn::Url => "Url",
        }
    }
}

impl TableViewItem<BasicColumn> for Channel {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Url => format!("{}", self.url),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::Url => self.url.cmp(&other.url),
        }
    }
}

pub fn render_window() {
    let mut siv = cursive::default();
    //let mut siv = Cursive::new();

    siv.load_toml(include_str!("style.toml")).unwrap();

    let mut table = TableView::<Channel, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c.width_percent(30))
        .column(BasicColumn::Url, "URL", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(20)
        });
        
    let mut items = Vec::new();
    
    let database_connection = database_handler::fetch_database_connection();
    let channels_result = database_handler::fetch_channels_vec(&database_connection);
    if channels_result.is_ok() {
        for channel in channels_result.unwrap() {
            items.push(Channel {
                id: channel.id,
                channel_id: channel.channel_id,
                name: channel.name,
                url: channel.url
            });
        }
    }

    table.set_items(items);

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name("table", move |table: &mut TableView<Channel, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

        let _ = videos_window::main();

        // siv.add_layer(
        //     Dialog::around(TextView::new(value))
        //         .title(format!("Removing row # {}", row))
        //         .button("Close", move |s| {
        //             s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
        //                 table.remove_item(index);
        //             });
        //             s.pop_layer();
        //         }),
        // );
    });


    siv.add_layer(Dialog::around(table.with_name("table").min_size((500, 200))).title("Channels View"));

    siv.add_global_callback('a', |s| s.add_layer(
        OnEventView::new(
            Dialog::new()
                .title("Add Channel URL")
                .content(
                    EditView::new()
                        .on_submit(add_url_event)
                        .with_name("edit")
                        .min_width(50),
                )
                .button("Ok", |s| {
                    let text = s
                        .call_on_name("edit", |view: &mut EditView| view.get_content())
                        .unwrap();
                    add_url_event(s, &text);
                })
                .dismiss_button("Cancel"),
        )
        .on_event(Event::Key(Key::Esc), |s| {
            s.pop_layer();
        }),
    ));


    siv.add_global_callback('t', |s| s.add_layer(
        // Most views can be configured in a chainable way
        Dialog::around(TextView::new("Hello Dialog!").min_width(100))
            .title("Cursive")
            .button("Foo", |_s| ())
            .button("Quit", |s| s.quit())
            .wrap_with(CircularFocus::new)
            .wrap_tab(),
    ));

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn add_url_event(siv: &mut Cursive, url: &str) {
    // do nothing for now
    let option_channel = channel_parser::fetch_channel_information(url);
    if option_channel.is_none() {
        // TODO: show error
        siv.pop_layer();    
    }

    let database_connection = database_handler::fetch_database_connection();
    let _ = database_handler::save_channel_info(&database_connection, &option_channel.unwrap());

    siv.pop_layer();
}