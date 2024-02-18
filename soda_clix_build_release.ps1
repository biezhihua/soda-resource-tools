cd ./soda_resource_tools_lib 
cargo build && cargo publish

cd ..
cd ./soda_cli
cargo build && cargo publish

cross build --target x86_64-apple-darwin --release
cross build --target x86_64-pc-windows-gnu --release
cross build --target x86_64-unknown-linux-gnu --release

tar -cvzf .\target\soda_clix_x86_64-apple-darwin.tar.gz .\target\x86_64-apple-darwin\release\soda_clix
tar -cvzf .\target\soda_clix_x86_64-pc-windows-gnu.tar.gz .\target\x86_64-pc-windows-gnu\release\soda_clix.exe
tar -cvzf .\target\soda_clix_x86_64-unknown-linux-gnu.tar.gz .\target\x86_64-unknown-linux-gnu\release\soda_clix