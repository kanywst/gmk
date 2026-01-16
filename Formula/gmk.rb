class Gmk < Formula
  desc "🚀 Bookmark & Interactive Git Clone Tool"
  homepage "https://github.com/kanywst/gmk"
  url "https://github.com/kanywst/gmk/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_REAL_SHA256_AFTER_RELEASE"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/gmk", "--version"
  end
end