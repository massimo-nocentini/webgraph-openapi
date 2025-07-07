
up:
	COMPOSE_BAKE=true docker compose up

build:
	COMPOSE_BAKE=true docker compose build

down:
	COMPOSE_BAKE=true docker compose down

refresh: build up

novolumes:
	COMPOSE_BAKE=true docker compose -f compose-novolumes.yml down
	COMPOSE_BAKE=true docker compose -f compose-novolumes.yml build
	COMPOSE_BAKE=true docker compose -f compose-novolumes.yml up