require 'open3'

module Stage0

    def run_test
        cmd = "cargo run --release --"
        stdout, stderr, status = Open3.capture3(cmd)

        if stdout.strip == "Hello, world!"
            puts "Output matches: #{stdout.strip}"
        else
            puts "Output does not match: #{stdout.strip}"
            exit 1
        end
    end

end
