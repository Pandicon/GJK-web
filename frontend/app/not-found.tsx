import Footer from "@/components/footer";
import Navbar from "@/components/navbar";

export default function NotFound() {
  return (
    <div className="flex flex-col min-h-[100vh] w-full">
      <Navbar />
      <div className="flex-grow grid justify-center items-center bg-secondary">
        <h2>404: Stránka nebyla nalezena</h2>
      </div>
      <Footer />
    </div>
  );
}
