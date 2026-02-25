
up:
	docker compose up -d

build:
	docker compose build --no-cache

down:
	docker compose down

refresh: down build up

novolumes:
	docker compose -f compose-novolumes.yml down
	docker compose -f compose-novolumes.yml build
	docker compose -f compose-novolumes.yml up

test-neo4j-bitcoin:
	curl --json @bitcoin-data.json http://localhost:8800/neo4j/db/paymentsgraph/tx/commit | jq

scratch:
	exit 0

	# https://neo4j.com/docs/query-api/current/query/
	curl --json @ethereum-data.json http://localhost:7474/db/model1/query/v2 | jq

	# https://neo4j.com/docs/http-api/5/query/
	curl --json @bitcoin-data.json http://localhost:7474/db/paymentsgraph/tx/commit | jq

	docker run -it --rm -v /Users/mn/Data/dltfruit/ardittiBACKUP/neo4j/data:/data -p 7474:7474 -e NEO4J_AUTH=none -e NEO4J_ACCEPT_LICENSE_AGREEMENT=yes --entrypoint bash neo4j:5.17.0-enterprise