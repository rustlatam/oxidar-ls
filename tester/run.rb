require 'tomlrb'
require_relative 'stage_0'

# Build the project
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

ensure_build

stage = select_stage
case stage
    when '0'
        include Stage0
        Stage0.run_test
    else
        puts "Unknown stage: #{stage}"
        exit 1
end

puts "Keep it up!"
