all: access_manager

access_manager: 
	cd access-manager && $(MAKE)
	cp access_manager/contract.wasm.gz ./access-manager.wasm.gz