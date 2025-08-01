class Dui < Formula
  desc "An intuitive Docker management CLI built in Rust"
  homepage "https://github.com/ukhan1219/dui"
  url "https://github.com/ukhan1219/dui/archive/refs/tags/v3.6.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256" # This will be updated with actual SHA256
  license "MIT"
  head "https://github.com/ukhan1219/dui.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/dui", "--help"
  end
end 