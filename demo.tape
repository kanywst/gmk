# Output the GIF to the assets directory
Output assets/demo.gif

# --- 🎨 Visual Styling ---
Set FontSize 20
Set Width 1200
Set Height 640
Set Padding 40
Set Margin 20
Set MarginFill "#252932"
Set BorderRadius 10
Set WindowBar Colorful
Set WindowBarSize 40
Set Theme "Catppuccin Mocha"

# --- 🎬 Scenario Setup ---
Hide
Type "export PATH=$PWD/target/release:$PATH"
Enter
Type "rm -f ~/.gmk/config.toml"
Enter
Type "clear"
Enter
Show

# --- 🚀 Scene 1: Introduction ---
Type "# Quick setup: bookmark your favorite repos"
Sleep 1s
Enter

# --- 🚀 Scene 2: Register multiple repos ---
Type "gmk set https://github.com/charmbracelet/vhs.git"
Enter
Sleep 500ms
Type "gmk set https://github.com/lotabout/skim.git"
Enter
Sleep 500ms
Type "gmk set https://github.com/rust-lang/cargo.git"
Enter
Sleep 1.5s

# --- 🚀 Scene 3: Interactive Clone ---
Type "clear"
Enter
Type "# Now, let's clone one interactively"
Enter
Sleep 1s

Type "gmk"
Sleep 500ms
Enter  # 👈 Press Enter to start gmk
Sleep 1s

# Type to filter (searching for 'skim' among 3 repos)
Type "ski"
Sleep 1s
# Select 'skim'
Enter
Sleep 3s

# --- 🏁 Cleanup ---
Hide
Type "rm -rf skim"
Enter