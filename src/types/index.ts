export interface Horse {
  id: string;
  name: string;
  breed: string;
  color: string;
  nationality: string;
  age: number;
  gender: "Male" | "Female";
  weight: number;
  height: number;
  length: number;
}

export interface User {
  id: string;
  email: string;
  password: string;
  username: string;
  created_at: number;
  updated_at: number;
}

export interface Stable {
  id: number;
  name: string;
  address: string;
  monthly_fee: number;
  created_at: number;
  updated_at: number;
}

export type StableCreate = Pick<Stable, "name" | "address" | "monthly_fee">;

