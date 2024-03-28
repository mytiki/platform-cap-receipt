.PHONY: compile build delete clean

compile: src/state_machine/state_machine.json $(wildcard src/state_machine/states/*.json)
	mkdir -p out
	set -x; \
    jq -s '.[0] as $$base | .[1:] | reduce .[] as $$state ($$base; .States += $$state)' $^ > out/state_machine.json
	cd infra && sam validate --lint

build:
	cd infra && sam build $(flags)

package:
	cd infra && sam package $(flags)

deploy:
	cd infra && sam deploy $(flags)

delete:
	cd infra && sam delete $(flags)

clean:
	rm -rf out
	rm -rf infra/.aws-sam
