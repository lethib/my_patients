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
