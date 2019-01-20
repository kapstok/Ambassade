# Voorbereiding
cargo build # in ambassade dir.
sudo cp -v ./target/debug/ambassade /usr/local/bin/ambassade
sudo npm i --global http-server

# Initializatie:
git init example
ambassade init

# Een 'module' ergens vandaan halen zonder een config file in de module zelf aan te maken:
ambassade hide elasticsearch-6.5.4 wget https://artifacts.elastic.co/downloads/elasticsearch/elasticsearch-6.5.4.tar.gz

# Modules toevoegen die al een config file hebben:
ambassade add Ambassade-example-login-stats git clone https://pagure.io/Ambassade-example-login-stats

# NPM packages werken nog niet zo lekker:
ambassade add Ambassade-example-login-user git clone https://pagure.io/Ambassade-example-login-user
pushd ./dep/Ambassade-example-login-user
npm i
popd

# Hide past het 'add' behavior aan:
ambassade hide requests git clone git://github.com/requests/requests.git --depth=1

# Bouw het project:
ambassade build
# Build command voor requests: pip install ./dep/requests --user
# Build command voor elasticsearch: tar -xvzf ./dep/elasticsearch-6.5.4.tar.gz -C ./dep
# Build command voor example: echo no build command

# Set run command for elasticsearch:
ambassade run elasticsearch-6.5.4

# Verander commando voor Windows:
nano ./dep_config/elasticsearch-6.5.4

# Run het project:
ambassade run elasticsearch-6.5.4 Ambassade-example-login-user


# Open browser zonder CORS:
about:config -> security.fileuri.strict_origin_policy -> false. # en daarna weer reverten ofc.

# Run het beheersysteem:
ambassade run Ambassade-example-login-stats
