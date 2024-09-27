/** @type {import('next').NextConfig} */
const nextConfig = {
  async redirects() {
    return [
      {
        source: "/login",
        destination: "http://localhost:2357/auth/redirect",
        permanent: true,
      },
    ];
  },
};

export default nextConfig;
