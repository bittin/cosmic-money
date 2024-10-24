use chrono::{Datelike, Duration, Local, NaiveDate};
use cosmic::{
    iced::{Alignment, Length, Padding},
    widget::{self, horizontal_space},
    Command, Element,
};

use crate::{
    app::Message,
    fl,
    models::{Category, NewCategory, UpdateCategory},
    STORE,
};

#[derive(Debug, Clone)]
pub enum CategoriesMessage {
    Update,
    AddCategory,
    NewCategoryNameChanged(String),
    NewCategoryDescriptionChanged(String),
    NewCategorySubmitted,
    NewCategoryCancel,
    EditCategoryName(String),
    EditCategoryDescription(String),
    EditCategoryCancel,
    EditCategorySubmitted,
    EditCategoryTypeChanged(usize),
    NewCategoryTypeChanged(usize),
    PreviousMonth,
    NextMonth,
    EditCategory(i32),
}

pub struct Categories {
    categories: Vec<Category>,
    add_category_view_active: bool,
    form_new_category_name: String,
    form_new_category_description: String,
    edit_category_form_name: String,
    edit_category_form_description: String,
    view_month: u32,
    view_year: i32,
    category_types_options: Vec<String>,
    selected_category_type: Option<usize>,
    edit_category_type: Option<usize>,
    edit_category_id: Option<i32>,
}

impl Default for Categories {
    fn default() -> Self {
        let mut store = STORE.lock().unwrap();
        let categories = store.get_categories();
        let now = Local::now();
        Self {
            categories: if let Ok(cat) = categories {
                cat
            } else {
                vec![]
            },
            add_category_view_active: false,
            form_new_category_name: "".to_string(),
            form_new_category_description: "".to_string(),
            view_month: now.month(),
            view_year: now.year(),
            category_types_options: vec![fl!("income"), fl!("expense")],
            selected_category_type: Some(0),
            edit_category_id: None,
            edit_category_form_name: "".to_string(),
            edit_category_form_description: "".to_string(),
            edit_category_type: Some(0),
        }
    }
}

impl Categories {
    pub fn add_category_view<'a>(&'a self) -> Element<'a, CategoriesMessage> {
        let mut element = widget::column();
        element = element.push(widget::vertical_space(Length::from(10)));
        element = element.push(
            widget::container(
                widget::column()
                    .push(widget::text::title4(fl!("new-category")))
                    .push(widget::vertical_space(Length::from(10)))
                    .push(
                        widget::row().push(
                            widget::column()
                                .push(widget::text::text(fl!("category-name")))
                                .push(widget::vertical_space(Length::from(3)))
                                .push(
                                    cosmic::widget::text_input(
                                        fl!("new-category"),
                                        &self.form_new_category_name,
                                    )
                                    .on_input(CategoriesMessage::NewCategoryNameChanged),
                                ),
                        ),
                    )
                    .push(widget::vertical_space(Length::from(10)))
                    .push(
                        widget::row().push(
                            widget::column()
                                .push(widget::text::text(fl!("category-description")))
                                .push(widget::vertical_space(Length::from(3)))
                                .push(
                                    cosmic::widget::text_input(
                                        fl!("category-description"),
                                        &self.form_new_category_description,
                                    )
                                    .on_input(CategoriesMessage::NewCategoryDescriptionChanged),
                                ),
                        ),
                    )
                    .push(widget::vertical_space(Length::from(10)))
                    .push(widget::text::text(fl!("category-type")))
                    .push(widget::dropdown(
                        &self.category_types_options,
                        self.selected_category_type,
                        CategoriesMessage::NewCategoryTypeChanged,
                    ))
                    .push(widget::vertical_space(Length::from(10)))
                    .push(
                        widget::row()
                            .push(
                                widget::button::text(fl!("add-category"))
                                    .on_press(CategoriesMessage::NewCategorySubmitted)
                                    .style(widget::button::Style::Suggested),
                            )
                            .push(widget::horizontal_space(Length::from(10)))
                            .push(
                                widget::button::text(fl!("cancel"))
                                    .on_press(CategoriesMessage::NewCategoryCancel)
                                    .style(widget::button::Style::Destructive),
                            )
                            .width(Length::Fill)
                            .align_items(Alignment::End),
                    )
                    .width(Length::Fill),
            )
            .padding(10)
            .width(Length::Fill)
            .style(cosmic::theme::Container::Card),
        );

        element = element.push(widget::vertical_space(Length::from(10)));

        element.into()
    }

    pub fn category_card<'a>(&'a self, c: &Category) -> Element<'a, CategoriesMessage> {
        let mut main_col = widget::column();
        let mut info_col = widget::column()
            .push(widget::text::title4(c.name.clone()))
            .push(widget::text::text(c.category_description.clone()))
            .push(widget::text::text(format!(
                "{}: {}",
                fl!("balance"),
                self.calculate_by_category_id(c.id).to_string(),
            )))
            .width(Length::Fill);

        let mut row = widget::row().push(info_col).push(
            widget::column()
                .push(
                    widget::button::icon(widget::icon::from_name("edit-symbolic"))
                        .on_press(CategoriesMessage::EditCategory(c.id)),
                )
                .align_items(Alignment::End)
                .width(Length::Fill),
        );

        main_col = main_col.push(row);

        if self.edit_category_id == Some(c.id) {
            main_col = main_col.push(widget::vertical_space(Length::from(10)));
            main_col = main_col.push(widget::divider::horizontal::default());
            main_col = main_col.push(widget::vertical_space(Length::from(10)));
            main_col = main_col.push(
                widget::row().push(
                    widget::column()
                        .push(widget::text::text(fl!("category-name")))
                        .push(
                            widget::text_input(fl!("category-name"), &self.edit_category_form_name)
                                .on_input(CategoriesMessage::EditCategoryName),
                        )
                        .push(widget::vertical_space(Length::from(10)))
                        .push(widget::text::text(fl!("category-description")))
                        .push(
                            widget::text_input(
                                fl!("category-description"),
                                &self.edit_category_form_description,
                            )
                            .on_input(CategoriesMessage::EditCategoryDescription),
                        )
                        .push(widget::vertical_space(Length::from(10)))
                        .push(widget::dropdown(
                            &self.category_types_options,
                            self.edit_category_type,
                            CategoriesMessage::EditCategoryTypeChanged,
                        ))
                        .push(widget::vertical_space(Length::from(10)))
                        .push(
                            widget::row()
                                .push(
                                    widget::button::text(fl!("save"))
                                        .on_press(CategoriesMessage::EditCategorySubmitted)
                                        .style(widget::button::Style::Suggested),
                                )
                                .push(widget::horizontal_space(Length::from(10)))
                                .push(
                                    widget::button::text(fl!("cancel"))
                                        .on_press(CategoriesMessage::EditCategoryCancel)
                                        .style(widget::button::Style::Destructive),
                                ),
                        ),
                ),
            );
        }

        let element = widget::container(main_col)
            .padding(10)
            .style(cosmic::theme::Container::Card);

        element.into()
    }

    pub fn view<'a>(&'a self) -> Element<'a, CategoriesMessage> {
        let mut element = widget::column()
            .padding(Padding::new(10.))
            .width(Length::Fill)
            .align_items(Alignment::Start);

        element = element.push(
            widget::row()
                .push(
                    widget::column()
                        .push(widget::text::title1(fl!("page_categories")))
                        .width(Length::Fill),
                )
                .push(
                    widget::column()
                        .push(
                            widget::row().push(
                                widget::button::text(fl!("add-category"))
                                    .on_press(CategoriesMessage::AddCategory)
                                    .style(widget::button::Style::Suggested),
                            ),
                        )
                        .width(Length::Fill)
                        .align_items(Alignment::End),
                ),
        );

        if self.add_category_view_active {
            element = element.push(self.add_category_view());
        }

        let month_names = vec![
            fl!("month-1"),  // January
            fl!("month-2"),  // February
            fl!("month-3"),  // March
            fl!("month-4"),  // April
            fl!("month-5"),  // May
            fl!("month-6"),  // June
            fl!("month-7"),  // July
            fl!("month-8"),  // August
            fl!("month-9"),  // September
            fl!("month-10"), // October
            fl!("month-11"), // November
            fl!("month-12"), // December
        ];

        element = element.push(
            widget::column()
                .push(
                    widget::row()
                        .push(
                            widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                                .on_press(CategoriesMessage::PreviousMonth),
                        )
                        .push(horizontal_space(Length::from(10)))
                        .push(
                            widget::container(
                                widget::row()
                                    .push(widget::text::text(
                                        month_names[self.view_month as usize - 1].clone(),
                                    ))
                                    .push(horizontal_space(Length::from(5)))
                                    .push(widget::text::text(self.view_year.to_string())),
                            )
                            .padding(Padding::from(7)),
                        )
                        .push(horizontal_space(Length::from(10)))
                        .push(
                            widget::button::icon(widget::icon::from_name("go-next-symbolic"))
                                .on_press(CategoriesMessage::NextMonth),
                        ),
                )
                .align_items(Alignment::Center)
                .width(Length::Fill),
        );

        element = element.push(widget::vertical_space(Length::from(10)));

        element = element.push(widget::text::title4(fl!("income-categories")));

        for c in &self
            .categories
            .clone()
            .into_iter()
            .filter(|c| c.is_income)
            .collect::<Vec<Category>>()
        {
            element = element
                .push(self.category_card(c))
                .push(widget::vertical_space(Length::from(10)));
        }

        element = element.push(widget::vertical_space(Length::from(10)));

        element = element.push(widget::text::title4(fl!("expense-categories")));

        for c in &self
            .categories
            .clone()
            .into_iter()
            .filter(|c| !c.is_income)
            .collect::<Vec<Category>>()
        {
            element = element
                .push(self.category_card(c))
                .push(widget::vertical_space(Length::from(10)));
        }

        widget::scrollable(element).into()
    }

    pub fn update(&mut self, message: CategoriesMessage) -> Command<crate::app::Message> {
        let mut commands = vec![];
        match message {
            CategoriesMessage::Update => {
                log::info!("updating category page");
                let mut store = STORE.lock().unwrap();
                let categories = store.get_categories();
                if let Ok(categories) = categories {
                    self.categories = categories;
                }
            }
            CategoriesMessage::AddCategory => {
                self.add_category_view_active = true;
            }
            CategoriesMessage::NewCategoryNameChanged(value) => {
                self.form_new_category_name = value;
            }
            CategoriesMessage::NewCategoryDescriptionChanged(value) => {
                self.form_new_category_description = value;
            }
            CategoriesMessage::NewCategorySubmitted => {
                let new_category = NewCategory {
                    name: self.form_new_category_name.as_str(),
                    is_income: self.selected_category_type == Some(0),
                    category_description: self.form_new_category_description.clone(),
                };
                let mut store = STORE.lock().unwrap();
                let _ = store.create_category(&new_category);
                self.add_category_view_active = false;
                self.form_new_category_name = "".to_string();
                commands.push(Command::perform(async {}, |_| {
                    Message::Categories(super::categories::CategoriesMessage::Update)
                }));
            }
            CategoriesMessage::EditCategoryName(value) => {
                self.edit_category_form_name = value;
            }
            CategoriesMessage::EditCategoryDescription(value) => {
                self.edit_category_form_description = value;
            }
            CategoriesMessage::EditCategoryTypeChanged(value) => {
                self.edit_category_type = Some(value)
            }
            CategoriesMessage::PreviousMonth => {
                if self.view_month == 1 {
                    self.view_month = 12;
                    self.view_year -= 1;
                } else {
                    self.view_month -= 1;
                }
            }
            CategoriesMessage::NextMonth => {
                if self.view_month == 12 {
                    self.view_month = 1;
                    self.view_year += 1;
                } else {
                    self.view_month += 1;
                }
            }
            CategoriesMessage::NewCategoryTypeChanged(value) => {
                self.selected_category_type = Some(value);
            }
            CategoriesMessage::NewCategoryCancel => {
                self.add_category_view_active = false;
                self.form_new_category_name = "".to_string();
            }
            CategoriesMessage::EditCategory(category_id) => {
                let category: Option<Category> = self
                    .categories
                    .clone()
                    .into_iter()
                    .find(|c| c.id == category_id);
                self.edit_category_id = Some(category_id);
                if let Some(category) = category {
                    self.edit_category_form_name = category.name;
                    self.edit_category_form_description = category.category_description;
                }
            }
            CategoriesMessage::EditCategoryCancel => {
                self.edit_category_id = None;
            }
            CategoriesMessage::EditCategorySubmitted => {
                log::info!("update category submitted");
                if let Some(id) = self.edit_category_id {
                    let update_category = UpdateCategory {
                        id,
                        name: &self.edit_category_form_name,
                        is_income: self.edit_category_type == Some(0),
                        category_description: self.edit_category_form_description.clone(),
                    };
                    let mut store = STORE.lock().unwrap();
                    let _ = store.update_category(&update_category);
                    self.edit_category_id = None;
                    commands.push(Command::perform(async {}, |_| {
                        Message::Categories(super::categories::CategoriesMessage::Update)
                    }));
                }
            }
        }
        Command::batch(commands)
    }

    fn calculate_by_category_id(&self, category_id: i32) -> f32 {
        let mut store = STORE.lock().unwrap();
        let (start_date, end_date) = self.get_month_start_and_end();
        match store.calculate_expense_by_category(category_id, &start_date, &end_date) {
            Ok(val) => val,
            Err(_) => 0.,
        }
    }

    fn get_month_start_and_end(&self) -> (NaiveDate, NaiveDate) {
        let month_start = NaiveDate::from_ymd_opt(self.view_year, self.view_month, 1)
            .expect("Data non valida per l'inizio del mese");

        let next_month = if self.view_month == 12 {
            NaiveDate::from_ymd_opt((self.view_month + 1) as i32, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(self.view_year, self.view_month + 1, 1)
        }
        .expect("Data non valida per il primo giorno del mese successivo");

        let month_end = next_month - Duration::days(1);

        (month_start, month_end)
    }
}
