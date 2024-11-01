import Footer from "@/components/footer";
import Navbar from "@/components/navbar";

export default function NotFound() {
  return (
    <>
      <Navbar />
      <div className="flex-grow grid justify-center items-center bg-secondary">
        <h2>404: Stránka nebyla nalezena</h2>
      </div>
      <Footer />
    </>
  );
}
