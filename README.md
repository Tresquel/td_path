# TDPath converter
Converts [TDPaths (.pth)](https://x4fx77x4f.github.io/dennispedia/teardown/tdpth.html) into lua files that can be parsed in-game.

Example output:
```lua
path = {
	version = '1.5.4',
	time = 24.725962,
	length = 1855,
	nodes = {
		{ -5.7933216, 8.561391, 6.787056 },
		{ -5.7933216, 8.561391, 6.787056 },
		{ -5.793324, 8.561359, 6.7870603 },
		{ -5.793324, 8.561372, 6.7870603 },
		{ -5.793324, 8.56139, 6.7870603 },
		{ -5.793324, 8.56139, 6.7870603 },
        -- rest of the nodes
    }
}
```

Contributions are welcome.

# Usage
Requirements:
- VCRedist
- Rust (required only if you want to build it yourself)

Just run the exe and it should convert all of your paths stored in `%LOCALAPPDATA%\Teardown\`