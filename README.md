# Fearly

Fearly is a personal emotional-state monitoring tool.  
The goal is simple: help people track how they feel, understand what influences their emotions, and spot patterns over time. It is intentionally minimal and honest. No gamification, no artificial positivity, no distractions.

This project exists to make self-observation easier for anyone who needs it.

---

## What Fearly does

- Lets you log your current emotional state in a structured, consistent way.
- Helps you build a timeline of entries so you can review how you’ve been feeling.
- Encourages awareness rather than judgement.
- Keeps the interface and workflow simple so it doesn’t get in the way.

Internally, the app is split into two parts:
- A backend located in `fearly/`
- A frontend client in `fearly_frontend/`

---

## Repository structure
```
fearly/
├── fearly/ # Backend service (API, logic, processing)
├── fearly_frontend/ # Frontend client (UI)
├── config.yml # Shared configuration
└── LICENSE # MIT license
```
---

## Tech used

Tool uses rust for backend and svelte for frontend, backend is made with help of JuliORM and Rocket.


---


## Configuration

The file `config.yml` centralizes environment and project settings.  
Use it for anything shared between backend, frontend, or CI pipelines.

---

## Roadmap

- Expand emotional categories and descriptions
- Add optional long-form notes per entry
- Add weekly and monthly trends
- Add simple data export
- Improve the frontend layout and onboarding
- Add testing and deployment setup

---

## License

This project is licensed under the MIT License.
