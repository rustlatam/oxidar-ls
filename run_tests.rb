require 'tomlrb'

# Helper functions
def ensure_build
    system "cargo build --release"
    if $?.exitstatus != 0
        puts "Oops, we could not build the project!"
        exit 1
    end
end

def select_stage
    toml_data = Tomlrb.load_file('Course.toml')
    return toml_data['test']['level']
end

# Build the project
puts "Building the project..."
ensure_build

# Run the tests
puts "Running tests..."
stage = select_stage
system "cargo test --release --features stage#{stage}"

# Congrats!
puts "All Done!"
