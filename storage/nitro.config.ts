//https://nitro.unjs.io/config
export default defineNitroConfig({
    srcDir: "server",
    compatibilityDate: "2025-04-19",
    routeRules: {
        "/**": { cors: true, headers: { 'Access-Control-Allow-Origin': '*', 'Access-Control-Allow-Headers': '*' } },
    }
});
