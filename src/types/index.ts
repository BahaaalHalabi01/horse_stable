export interface Horse {
  id: String;
  name: String;
  breed: String;
  color: String;
  nationality: String;
  age: Number;
  gender: "Male" | "Female";
  weight: Number;
  height: Number;
  length: Number;
}

export interface User {
  id: String;
  email: String;
  password: String;
  username: String;
  created_at: Number;
  updated_at: Number;
}
