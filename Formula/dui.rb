class Dui < Formula
  desc "Docker management CLI built in Rust"
  homepage "https://github.com/ukhan1219/dui"
  url "https://github.com/ukhan1219/dui/archive/refs/tags/v4.1.11.tar.gz"
  sha256 "a7f8d511f1e6adfcd7a23f58119e1b36ee23860d3bbce1d03984f53f17512ad1"
  license "MIT"
  head "https://github.com/ukhan1219/dui.git", branch: "main"

  depends_on "rust" => :build

  livecheck do
    url :stable
    regex(/^v?(\d+(?:\.\d+)+)$/i)
  end

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test that the binary works and provides help
    assert_match "USAGE:", shell_output("#{bin}/dui --help")

    # Test that it can list containers (even if none exist)
    assert_match "containers", shell_output("#{bin}/dui containers --help")
  end
end 