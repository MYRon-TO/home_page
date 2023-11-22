use super::*;

/// ## handler for list page/type: all
pub async fn list_all(app_state: AppState) -> ListTemplate {
  let rows = app_state.database.lock().await.get_all_blog().await;
  let mut list = ListTemplate::new(ListType::All);
  for row in rows {
    let item = List::deal_db_date(row).await;
    list.items.append(item).await;
  }

  list
}

/// ## handler for list page/type: series
pub async fn list_series(app_state: AppState) -> ListTemplate {
  let rows = app_state.database.lock().await.get_all_series().await;
  let mut list = ListTemplate::new(ListType::Series);
  for row in rows {
    let item = List::deal_db_date(row).await;
    list.items.append(item).await;
  }

  list
}

/// ## handler for list page/type: tags
pub async fn list_tags(app_state: AppState) -> ListTemplate {
  let rows = app_state.database.lock().await.get_all_series().await;
  let mut list = ListTemplate::new(ListType::Tags);
  for row in rows {
    let item = List::deal_db_date(row).await;
    list.items.append(item).await;
  }

  list
}

pub async fn list_nil() -> ListTemplate {
  ListTemplate{
    items: List{
      content :vec![]
    },
    list_type: ListType::Nil,
  }
}
