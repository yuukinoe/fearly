import fs from 'fs';
import path from 'path';
import yaml from 'js-yaml';

export interface Config {
	database: {
		host: string;
		port: number;
		user: string;
		password: string;
		namespace: string;
		database: string;
	};

	user: {
		login: string;
		password: string;
	};

	directories: {
		media: string;
	};

	debug: {
		prints: boolean;
		cookies: boolean;
	};

	session_duration: number;

	routers: {
		api: RouterEntry[];
		web: (RouterEntry & { noport?: boolean })[];
		domain: string;
		routing: {
			csr: boolean;
		};
	};
}

export interface RouterEntry {
	protocol: string;
	hostname: string;
	port: number;
	pathname: string;
}

export function loadConfig(): Config {
	const configPath = path.resolve('../config.yml');
	const fileContents = fs.readFileSync(configPath, 'utf8');
	const data = yaml.load(fileContents) as Config;
	// console.log(data);
	return data;
}
