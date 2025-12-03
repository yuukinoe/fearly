<script lang="ts">
	import type { PageProps } from './$types.js';
	import type { Fear } from '$lib/types.ts';
	import { Dialog, AlertDialog, Slider, DatePicker, TimeField } from 'bits-ui';
	import { Subtitles } from 'phosphor-svelte';
	import CalendarBlank from 'phosphor-svelte/lib/CalendarBlank';
	import CaretLeft from 'phosphor-svelte/lib/CaretLeft';
	import CaretRight from 'phosphor-svelte/lib/CaretRight';
	import X from 'phosphor-svelte/lib/X';
	import cn from 'clsx';

	const { data }: PageProps = $props();
	import {
		getLocalTimeZone,
		toTime,
		toCalendarDate,
		fromDate,
		Time,
		CalendarDate
	} from '@internationalized/date';
	import { deleteFear, updateFear } from '$lib/client.ts';
	import { removeMilisFromTime } from '$lib/utilts.ts';
	import { invalidateAll } from '$app/navigation';

	let dialogOpen = $state(false);
	let currentItem = $state<Fear>({} as Fear);
	let editMode = $state(false);
	let deleteOpen = $state(false);
	let toDeleteID = $state<string>('');

	const handleDelete = async () => {
		await deleteFear(toDeleteID);
		toDeleteID = '';
		deleteOpen = false;
		dialogOpen = false;
		invalidateAll();
	};

	const handleUpdate = async () => {
		await updateFear(currentItem, calendarDate!.toString(), removeMilisFromTime(calendarTime!));
		dialogOpen = false;
		editMode = false;
		invalidateAll();
	};

	let calendarDate = $state<CalendarDate>();
	let calendarTime = $state<Time>();
</script>

<div
	class="grid gap-5 p-5 font-roboto text-white sm:p-10 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4"
>
	{#each data.data as item (item.id.id.String)}
		<button
			class="flex flex-col gap-2 rounded-lg bg-[#121212] p-5 text-left"
			onclick={() => {
				let dateTime = fromDate(new Date(item.datetime), getLocalTimeZone());
				calendarDate = toCalendarDate(dateTime);
				calendarTime = toTime(dateTime);
				currentItem = item;
				dialogOpen = true;
			}}
		>
			<span class="line-clamp-1 text-xs font-bold">{item.content}</span>
			<span class="line-clamp-1 font-bold">{item.title}</span>
			<div class="flex flex-col gap-1">
				<span class="line-clamp-2 text-gray-300">{item.reaction}</span>
				<div class="flex justify-between">
					<span class="text-xs text-gray-400"
						>&bull; {new Date(item.datetime).toLocaleString()}</span
					>
					<span class="text-xs text-gray-400">{item.emotion}/100</span>
				</div>
			</div>
		</button>
	{/each}
</div>
<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Portal>
		<Dialog.Overlay
			class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/80 font-roboto"
		/>
		<Dialog.Content
			class="dark data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-50 h-full max-h-[850px]  w-full  translate-x-[-50%] translate-y-[-50%] overflow-y-scroll border bg-background shadow-popover outline-hidden sm:max-w-[490px] sm:rounded-card-lg sm:p-5 md:w-full"
		>
			<div class="h-full w-full max-w-[900px] p-5 text-white sm:rounded-xl sm:bg-[#121212]">
				<div class="flex flex-col gap-5 pb-8">
					<div class="flex flex-col gap-2">
						<label class="font-roboto text-sm font-medium" for="title">Sytuacja</label>
						<div class="flex w-full">
							<div
								class="flex items-center justify-center rounded-tl rounded-bl border-t border-b border-l border-solid border-[#343434] bg-[#0d0d0d] p-2 px-3"
							>
								<Subtitles class="" />
							</div>
							<input
								bind:value={currentItem.title}
								type="text"
								id="title"
								class="w-full rounded-t rounded-l-none rounded-r rounded-b rounded-bl-none border-t border-r border-b border-l-0 border-[#343434] bg-[#0d0d0d] focus:ring-[#696969]"
								placeholder="Tytuł"
								disabled={!editMode}
								required
							/>
						</div>
					</div>

					<div class="flex flex-col gap-2">
						<label class="font-roboto text-sm font-medium" for="content">Myśl</label>
						<div class="flex w-full">
							<textarea
								bind:value={currentItem.content}
								id="content"
								class="min-h-[200px] w-full rounded border-[#343434] bg-[#0d0d0d] focus:border-[#2b2b2b] focus:ring-[#696969] active:border-[#2b2b2b]"
								placeholder="Opis"
								disabled={!editMode}
								required
							>
							</textarea>
						</div>
					</div>
					<div class="flex flex-col gap-2">
						<label class="font-roboto text-sm font-medium" for="reaction">Zachowanie</label>
						<div class="flex w-full">
							<textarea
								bind:value={currentItem.reaction}
								id="reaction"
								class="min-h-[200px] w-full rounded border-[#343434] bg-[#0d0d0d] focus:border-[#2b2b2b] focus:ring-[#696969] active:border-[#2b2b2b]"
								placeholder="Zachowanie"
								disabled={!editMode}
								required
							>
							</textarea>
						</div>
					</div>
					<div class="flex w-full gap-2">
						<Slider.Root
							type="single"
							bind:value={currentItem.emotion}
							class="dark relative flex w-full touch-none items-center select-none"
							disabled={!editMode}
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
						<span>{currentItem.emotion}%</span>
					</div>
					<div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
						<DatePicker.Root
							weekdayFormat="short"
							fixedWeeks={true}
							bind:value={calendarDate}
							disabled={!editMode}
						>
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
											<div
												class="flex flex-col space-y-4 pt-4 sm:flex-row sm:space-y-0 sm:space-x-4"
											>
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

						<TimeField.Root bind:value={calendarTime} disabled={!editMode}>
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
						class="rounded-lg bg-white p-2 px-5 font-roboto font-bold text-black transition-all hover:opacity-90 active:scale-99"
						onclick={() => (editMode = !editMode)}
					>
						Tryb edycji
					</button>
					<button
						class={`rounded-lg bg-[#f77069] p-2 px-5 font-roboto font-bold text-black transition-all hover:opacity-90 active:scale-99 ${editMode ? '' : 'hidden'}`}
						onclick={() => {
							toDeleteID = currentItem.id.id.String;
							deleteOpen = true;
						}}
					>
						Usuń
					</button>
					<button
						class={`rounded-lg bg-white p-2 px-5 font-roboto font-bold text-black transition-all hover:opacity-90 active:scale-99 ${editMode ? '' : 'hidden'}`}
						onclick={handleUpdate}
					>
						Zapisz
					</button>
				</div>
			</div>
			<Dialog.Close
				class="absolute top-5 right-5 rounded-md focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:ring-offset-background focus-visible:outline-hidden active:scale-[0.98]"
			>
				<div>
					<X class="size-5 text-foreground" />
					<span class="sr-only">Close</span>
				</div>
			</Dialog.Close>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

<AlertDialog.Root bind:open={deleteOpen}>
	<AlertDialog.Portal>
		<AlertDialog.Overlay
			class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/80"
		/>
		<AlertDialog.Content
			class="dark data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-card-lg border bg-background p-7 shadow-popover outline-hidden sm:max-w-lg md:w-full "
		>
			<div class="flex flex-col gap-4 pb-6">
				<AlertDialog.Title class="font-roboto text-lg font-semibold tracking-tight text-white">
					Czy na pewno chcesz usunąć ten wpis?
				</AlertDialog.Title>
				<AlertDialog.Description class="font-roboto text-sm text-foreground-alt">
					Tej operacji nie można cofnąć, wpis zostanie usunięty. Czy chcesz kontynuować?
				</AlertDialog.Description>
			</div>
			<div class="flex w-full flex-col items-center justify-center gap-2 sm:flex-row">
				<AlertDialog.Cancel
					class="w-full rounded-lg bg-white p-2 px-5 font-roboto font-bold text-black transition-all hover:opacity-90 active:scale-99"
				>
					Anuluj
				</AlertDialog.Cancel>
				<AlertDialog.Action
					class="w-full rounded-lg bg-[#f77069] p-2 px-5 font-roboto font-bold text-white transition-all hover:opacity-90 active:scale-99"
					onclick={handleDelete}
				>
					Usuń
				</AlertDialog.Action>
			</div>
		</AlertDialog.Content>
	</AlertDialog.Portal>
</AlertDialog.Root>
