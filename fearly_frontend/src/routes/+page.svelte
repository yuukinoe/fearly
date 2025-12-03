<script lang="ts">
	import { loginUser, addFear as addFearToDatabase } from '$lib/client.ts';
	import { Password, Subtitles, User } from 'phosphor-svelte';
	import { DatePicker, TimeField, Slider } from 'bits-ui';
	import cn from 'clsx';

	import CalendarBlank from 'phosphor-svelte/lib/CalendarBlank';
	import CaretLeft from 'phosphor-svelte/lib/CaretLeft';
	import CaretRight from 'phosphor-svelte/lib/CaretRight';
	import {
		getLocalTimeZone,
		toTime,
		now,
		toCalendarDate,
		CalendarDate,
		Time
	} from '@internationalized/date';
	import type { PageProps } from './$types.js';
	import { invalidateAll } from '$app/navigation';
	import { removeMilisFromTime } from '$lib/utilts.ts';
	import { onDestroy } from 'svelte';
	const { data }: PageProps = $props();
	let userData = $state({
		username: '',
		password: ''
	});
	const login = async () => {
		const loginData = {
			username: userData.username,
			password: userData.password
		};
		await loginUser(loginData).then((response) => {
			if (response) {
				window.location.href = '/';
			}
		});
		return;
	};
	const emptyAddFear = {
		title: '',
		content: '',
		reaction: '',
		emotion: 50,
		date: '',
		time: ''
	};
	const tz = getLocalTimeZone();

	let addFear = $state(emptyAddFear);
	let calendarDate = $state<CalendarDate>();
	let calendarTime = $state<Time>();

	function updateDateTime() {
		const current = now(tz);
		calendarDate = toCalendarDate(current);
		calendarTime = toTime(current);
	}
	updateDateTime();

	const interval = setInterval(updateDateTime, 120_000);
	onDestroy(() => clearInterval(interval));

	async function addFearHandler() {
		addFear.date = calendarDate!.toString();
		addFear.time = removeMilisFromTime(calendarTime!);
		await addFearToDatabase(addFear);
		// await addFearToDatabase(fearData);
		invalidateAll();
	}
</script>

{#if !data.isLoggedIn}
	<div class="flex min-h-screen flex-col items-center justify-center">
		<div class="w-full max-w-[400px] rounded-xl border border-solid border-[#27272a] bg-[#121212]">
			<div class="flex w-full max-w-[800px] flex-col gap-10 p-5">
				<div class="flex flex-col gap-5 text-white">
					<div class="flex flex-col">
						<label class="font-roboto font-bold" for="username">Nazwa użytkownika</label>
						<div class="flex w-full">
							<div
								class="flex items-center justify-center rounded-tl rounded-bl border-t border-b border-l border-solid border-[#343434] bg-[#0d0d0d] p-2 px-3"
							>
								<User class="" />
							</div>
							<input
								bind:value={userData.username}
								type="text"
								id="username"
								class="w-full rounded-t rounded-l-none rounded-r rounded-b rounded-bl-none border-t border-r border-b border-l-0 border-[#343434] bg-[#0d0d0d] focus:ring-[#696969]"
								placeholder="Nazwa użytkownika"
								required
							/>
						</div>
					</div>
					<div class="flex flex-col">
						<label class="font-roboto font-bold" for="password">Hasło</label>
						<div class="flex w-full">
							<div
								class="flex items-center justify-center rounded-tl rounded-bl border-t border-b border-l border-solid border-[#343434] bg-[#0d0d0d] p-2 px-3"
							>
								<Password class="" />
							</div>
							<input
								bind:value={userData.password}
								type="password"
								id="password"
								class="w-full rounded-t rounded-l-none rounded-r rounded-b rounded-bl-none border-t border-r border-b border-l-0 border-[#343434] bg-[#0d0d0d] focus:ring-[#696969]"
								placeholder="Hasło"
								minlength="8"
								onkeydown={(e: KeyboardEvent) => {
									if (e.key === 'Enter') {
										login();
									}
								}}
								required
							/>
						</div>
					</div>
				</div>
				<button
					class=" rounded-lg bg-white p-2 px-5 font-roboto font-bold text-black transition-all hover:opacity-90 active:scale-99"
					onclick={login}
				>
					Zaloguj się
				</button>
			</div>
		</div>
	</div>
{:else}
	<div class="flex flex-col items-center">
		<div
			class="h-full min-h-screen w-full max-w-[900px] border-r border-l border-solid border-[#27272a] bg-[#121212] p-5 text-white"
		>
			<div class="flex flex-col gap-5">
				<div class="flex flex-col gap-2">
					<label class="font-roboto text-sm font-medium" for="title">Sytuacja</label>
					<div class="flex w-full">
						<div
							class="flex items-center justify-center rounded-tl rounded-bl border-t border-b border-l border-solid border-[#343434] bg-[#0d0d0d] p-2 px-3"
						>
							<Subtitles class="" />
						</div>
						<input
							bind:value={addFear.title}
							type="text"
							id="title"
							class="w-full rounded-t rounded-l-none rounded-r rounded-b rounded-bl-none border-t border-r border-b border-l-0 border-[#343434] bg-[#0d0d0d] focus:ring-[#696969]"
							placeholder="Tytuł"
							required
						/>
					</div>
				</div>

				<div class="flex flex-col gap-2">
					<label class="font-roboto text-sm font-medium" for="content">Myśl</label>
					<div class="flex w-full">
						<textarea
							bind:value={addFear.content}
							id="content"
							class="min-h-[200px] w-full rounded border-[#343434] bg-[#0d0d0d] focus:border-[#2b2b2b] focus:ring-[#696969] active:border-[#2b2b2b]"
							placeholder="Opis"
							required
						>
						</textarea>
					</div>
				</div>
				<div class="flex flex-col gap-2">
					<label class="font-roboto text-sm font-medium" for="reaction">Zachowanie</label>
					<div class="flex w-full">
						<textarea
							bind:value={addFear.reaction}
							id="reaction"
							class="min-h-[200px] w-full rounded border-[#343434] bg-[#0d0d0d] focus:border-[#2b2b2b] focus:ring-[#696969] active:border-[#2b2b2b]"
							placeholder="Zachowanie"
							required
						>
						</textarea>
					</div>
				</div>
				<div class="flex w-full gap-2">
					<Slider.Root
						type="single"
						bind:value={addFear.emotion}
						class="dark relative flex w-full touch-none items-center select-none"
					>
						<span
							class="relative h-2 w-full grow cursor-pointer overflow-hidden rounded-full bg-dark-10"
						>
							<Slider.Range class="absolute h-full bg-foreground" />
						</span>
						<Slider.Thumb
							index={0}
							class={cn(
								'block size-[25px] cursor-pointer rounded-full border border-border-input bg-background shadow-sm transition-colors hover:border-dark-40 focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50 data-active:scale-[0.98] data-active:border-dark-40 dark:bg-foreground dark:shadow-card'
							)}
						/>
					</Slider.Root>
					<span>{addFear.emotion}%</span>
				</div>
				<div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
					<DatePicker.Root weekdayFormat="short" fixedWeeks={true} bind:value={calendarDate}>
						<div class="dark flex w-full flex-col gap-1.5">
							<DatePicker.Label class="block text-sm font-medium select-none"
								><span class="font-roboto">Data</span></DatePicker.Label
							>
							<DatePicker.Input
								class="flex h-input w-full  items-center rounded-input border border-border-input bg-background px-2 py-3 text-sm tracking-[0.01em] text-foreground select-none focus-within:border-border-input-hover focus-within:shadow-date-field-focus hover:border-border-input-hover"
							>
								{#snippet children({ segments })}
									{#each segments as { part, value }, i (part + i)}
										<div class="inline-block select-none">
											{#if part === 'literal'}
												<DatePicker.Segment {part} class="p-1 text-muted-foreground">
													{value}
												</DatePicker.Segment>
											{:else}
												<DatePicker.Segment
													{part}
													class="rounded-5px px-1 py-1 hover:bg-muted focus:bg-muted focus:text-foreground focus-visible:ring-0! focus-visible:ring-offset-0! aria-[valuetext=Empty]:text-muted-foreground"
												>
													{value}
												</DatePicker.Segment>
											{/if}
										</div>
									{/each}
									<DatePicker.Trigger
										class="ml-auto inline-flex size-8 items-center justify-center rounded-5px text-foreground/60 transition-all hover:bg-muted active:bg-dark-10"
									>
										<CalendarBlank class="size-6" />
									</DatePicker.Trigger>
								{/snippet}
							</DatePicker.Input>
							<DatePicker.Content sideOffset={6} class="z-50">
								<DatePicker.Calendar
									class="rounded-15px border border-dark-10 bg-background-alt p-[22px] shadow-popover"
								>
									{#snippet children({ months, weekdays })}
										<DatePicker.Header class="flex items-center justify-between">
											<DatePicker.PrevButton
												class="inline-flex size-10 items-center justify-center rounded-9px bg-background-alt transition-all hover:bg-muted active:scale-[0.98]"
											>
												<CaretLeft class="size-6" />
											</DatePicker.PrevButton>
											<DatePicker.Heading class="text-[15px] font-medium" />
											<DatePicker.NextButton
												class="inline-flex size-10 items-center justify-center rounded-9px bg-background-alt transition-all hover:bg-muted active:scale-[0.98]"
											>
												<CaretRight class="size-6" />
											</DatePicker.NextButton>
										</DatePicker.Header>
										<div class="flex flex-col space-y-4 pt-4 sm:flex-row sm:space-y-0 sm:space-x-4">
											{#each months as month (month.value)}
												<DatePicker.Grid class="w-full border-collapse space-y-1 select-none">
													<DatePicker.GridHead>
														<DatePicker.GridRow class="mb-1 flex w-full justify-between">
															{#each weekdays as day (day)}
																<DatePicker.HeadCell
																	class="w-10 rounded-md text-xs font-normal! text-muted-foreground"
																>
																	<div>{day.slice(0, 2)}</div>
																</DatePicker.HeadCell>
															{/each}
														</DatePicker.GridRow>
													</DatePicker.GridHead>
													<DatePicker.GridBody>
														{#each month.weeks as weekDates (weekDates)}
															<DatePicker.GridRow class="flex w-full">
																{#each weekDates as date (date)}
																	<DatePicker.Cell
																		{date}
																		month={month.value}
																		class="relative size-10 p-0! text-center text-sm"
																	>
																		<DatePicker.Day
																			class="group relative inline-flex size-10 items-center justify-center rounded-9px border border-transparent bg-transparent p-0 text-sm font-normal whitespace-nowrap text-foreground transition-all hover:border-foreground data-disabled:pointer-events-none data-disabled:text-foreground/30 data-outside-month:pointer-events-none data-selected:bg-foreground data-selected:font-medium data-selected:text-background data-unavailable:text-muted-foreground data-unavailable:line-through"
																		>
																			<div
																				class="absolute top-[5px] hidden size-1 rounded-full bg-foreground transition-all group-data-selected:bg-background group-data-today:block"
																			></div>
																			{date.day}
																		</DatePicker.Day>
																	</DatePicker.Cell>
																{/each}
															</DatePicker.GridRow>
														{/each}
													</DatePicker.GridBody>
												</DatePicker.Grid>
											{/each}
										</div>
									{/snippet}
								</DatePicker.Calendar>
							</DatePicker.Content>
						</div>
					</DatePicker.Root>

					<TimeField.Root bind:value={calendarTime}>
						<div class="dark flex w-full flex-col gap-1.5">
							<TimeField.Label class="block text-sm font-medium select-none"
								><span class="font-roboto">Czas</span></TimeField.Label
							>
							<TimeField.Input
								name="time"
								class="flex h-input w-full items-center rounded-input border border-border-input bg-background px-2 py-3 text-sm tracking-[0.01em] text-foreground select-none focus-within:border-border-input-hover focus-within:shadow-date-field-focus hover:border-border-input-hover data-invalid:border-destructive "
							>
								{#snippet children({ segments })}
									{#each segments as { part, value }, i (part + i)}
										<div class="inline-block select-none">
											{#if part === 'literal'}
												<TimeField.Segment {part} class="p-1 text-muted-foreground">
													{value}
												</TimeField.Segment>
											{:else}
												<TimeField.Segment
													{part}
													class="rounded-5px px-1 py-1 hover:bg-muted focus:bg-muted focus:text-foreground focus-visible:ring-0! focus-visible:ring-offset-0! aria-[valuetext=Empty]:text-muted-foreground data-invalid:text-destructive"
												>
													{value}
												</TimeField.Segment>
											{/if}
										</div>
									{/each}
								{/snippet}
							</TimeField.Input>
						</div>
					</TimeField.Root>
				</div>
				<button
					class=" rounded-lg bg-white p-2 px-5 font-roboto font-bold text-black transition-all hover:opacity-90 active:scale-99"
					onclick={addFearHandler}
				>
					Dodaj
				</button>
			</div>
		</div>
	</div>
{/if}
