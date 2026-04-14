# TODO

## tw-animate-css Import Issue

**Status:** ✅ Resolved

**Solution:**
Added the correct import path in `static/css/tailwind.css`:
```css
@import "../../node_modules/tw-animate-css/dist/tw-animate.css";
```

The tw-animate-css package exports CSS directly from its dist folder. Tailwind v4 resolves relative paths from the CSS file location, so the path `../../node_modules/...` correctly traverses from `static/css/` back to the workspace root.

**Build Result:**
✅ `bash build.sh` completes successfully with no import errors

**Related Files:**
- `static/css/tailwind.css` (line 2)
- `package.json` (has dependency)
