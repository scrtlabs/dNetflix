all: access_manager

access_manager: 
	cd access-manager && $(MAKE)
	cp access-manager/contract.wasm ./access_manager.wasm
	