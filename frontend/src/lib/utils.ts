import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatAccessKey(accessKey: string): string {
  // Remove any characters except alphanumeric and hyphens, then return first 15 characters
  return accessKey
    .replace(/[^a-zA-Z0-9-]/g, "")
    .slice(0, 15)
    .toUpperCase();
}

export const formatSSN = (ssn: string): string =>
  `${ssn[0]} ${ssn.slice(1, 3)} ${ssn.slice(3, 5)} ${ssn.slice(5, 7)} ${ssn.slice(7, 10)} ${ssn.slice(10, 13)} ${ssn.slice(13, 15)}`;
