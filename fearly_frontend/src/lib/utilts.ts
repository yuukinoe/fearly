import { Time } from '@internationalized/date';

export function removeMilisFromTime(calendarTime: Time): string {
	const h = calendarTime.hour.toString().padStart(2, '0');
	const m = calendarTime.minute.toString().padStart(2, '0');
	const s = calendarTime.second.toString().padStart(2, '0');

	return `${h}:${m}:${s}`;
}
