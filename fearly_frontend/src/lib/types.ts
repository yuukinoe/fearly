export type IDWrapper = {
	tb: string;
	id: {
		String: string;
	};
};

export interface GenericResponse<T> {
	status: number;
	message: T;
}

export interface VecData<T> {
	VecData: T[];
}

export interface AddFear {
	title: string;
	content: string;
	reaction: string;
	emotion: number;
	date: string;
	time: string;
}

export interface Fear {
	id: IDWrapper;
	title: string;
	reaction: string;
	emotion: number;
	content: string;
	datetime: string;
}
