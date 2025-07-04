// Cloudflare Worker entry point
export default {
  async fetch(request, env, ctx) {
    // Serve static assets from the dist directory
    return env.ASSETS.fetch(request);
  },
};
