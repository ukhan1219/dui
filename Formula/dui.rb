class Dui < Formula
  desc "An intuitive Docker management CLI built in Rust"
  homepage "https://github.com/ukhan1219/dui"
  version "3.1.0"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/ukhan1219/dui/releases/download/v#{version}/dui-macos-aarch64"
      sha256 "PLACEHOLDER_SHA256"
    else
      url "https://github.com/ukhan1219/dui/releases/download/v#{version}/dui-macos-x86_64"
      sha256 "PLACEHOLDER_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/ukhan1219/dui/releases/download/v#{version}/dui-linux-aarch64"
      sha256 "PLACEHOLDER_SHA256"
    else
      url "https://github.com/ukhan1219/dui/releases/download/v#{version}/dui-linux-x86_64"
      sha256 "PLACEHOLDER_SHA256"
    end
  end

  def install
    if OS.mac?
      if Hardware::CPU.arm?
        bin.install "dui-macos-aarch64" => "dui"
      else
        bin.install "dui-macos-x86_64" => "dui"
      end
    else
      if Hardware::CPU.arm?
        bin.install "dui-linux-aarch64" => "dui"
      else
        bin.install "dui-linux-x86_64" => "dui"
      end
    end
  end

  test do
    system "#{bin}/dui", "--version"
  end
end 