/// <reference types="vite/client" />

interface ImportMetaEnv {
	VITE_BASE_API_URL: string;
}

// biome-ignore lint/correctness/noUnusedVariables: Vite env types
interface ImportMeta {
	readonly env: ImportMetaEnv;
}
