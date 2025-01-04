export type CreateUserInput = {
  ask_for_new_password: boolean,
  birth_date: string,
  email: string,
  first_name: string,
  last_name: string,
  password: string,
  username: string,
}

export type ListUserOutput = {
  id: string,
  ask_for_new_password: boolean,
  email: string,
  first_name: string,
  last_name: string,
  password_hash: string,
  username: string,
}
