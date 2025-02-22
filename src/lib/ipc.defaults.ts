export enum Commands {
  feed_horse = "feed_horse",
  get_current_user = "get_current_user",
  list_stables = "list_stables",
  list_all_horses = "list_all_horses",
  add_horse = "add_horse",
  edit_horse = "edit_horse",
  remove_horse = "remove_horse",
  register_user = "register_user",
  login = "login",
  get_horse = "get_horse",
  create_stable = "create_stable",
}


const currency = new Intl.NumberFormat('ru-RU', {
  style: 'currency',
  currency: 'RUB',
});

export const formatter = {
  currency,
}
