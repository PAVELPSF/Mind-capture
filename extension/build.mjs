import * as esbuild from "esbuild";
import { mkdirSync, cpSync, existsSync } from "node:fs";

const outdir = "dist";
const watch = process.argv.includes("--watch");

if (!existsSync(outdir)) mkdirSync(outdir, { recursive: true });

const ctx = await esbuild.context({
  entryPoints: ["src/background.ts", "src/popup/popup.ts", "src/content.ts"],
  outdir,
  bundle: true,
  format: "esm",
  target: "es2020",
  platform: "browser",
  sourcemap: false,
  minify: false,
});

// Copy static files
cpSync("src/popup/popup.html", `${outdir}/popup.html`);
if (existsSync("icons")) cpSync("icons", `${outdir}/icons`, { recursive: true });

if (watch) {
  console.log("Watching for changes...");
  await ctx.watch();
} else {
  await ctx.rebuild();
  console.log("Extension built successfully.");
  await ctx.dispose();
}
