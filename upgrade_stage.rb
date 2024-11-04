require 'toml-rb'

# Load the toml file
path = File.join(File.dirname(__FILE__), 'Course.toml')
toml_data = TomlRB.load_file(path)

# Upgrade the stage
stage = toml_data['test']['level']
next_stage = Integer(stage) + 1
puts "Upgrading to stage #{next_stage}..."
toml_data['test']['level'] = next_stage.to_s

# Save the file
data = TomlRB.dump(toml_data)
File.write(path, data)
puts "All done!"
