/** @type {import('next').NextConfig} */
const nextConfig = {
  async redirects() {
    return [
      {
        source: "/login",
        destination: `${process.env.API_HOST}/auth/redirect`,
        permanent: true,
      },
    ];
  },
};

export default nextConfig;
