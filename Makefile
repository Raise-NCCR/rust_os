baremetal:
	cargo build

linux:
	cargo rustc -- -C link-arg=-nostartfiles

windows:
	cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"

mac:
	cargo rustc -- -C link-args="-e __start -static -nostartfiles"