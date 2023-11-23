use super::*;

/// ## handler for list page/type: all
pub async fn list_blog(app_state: AppState, quest: QuestBlog) -> ListTemplate {
  let rows = match quest {
    QuestBlog::All => app_state.database.lock().await.get_all_blog().await,
    QuestBlog::Series(series) => app_state.database.lock().await.get_blog_from_series(&series).await,
    QuestBlog::Tags(tags) => app_state.database.lock().await.get_blog_from_tag(&tags).await,
  };
  let mut list = ListTemplate::new(ListType::Blog);
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
  let rows = app_state.database.lock().await.get_all_tags().await;
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
