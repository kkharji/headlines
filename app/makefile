bundle_debug:
	cargo bundle 

run_app: bundle_debug
	killall headlines_app ||:
	open ../target/debug/bundle/osx/Headlines.app

bundle_release:
	cargo bundle --release

run_release_app: bundle_release
	killall headlines_app ||:
	open ../target/debug/bundle/osx/Headlines.app
