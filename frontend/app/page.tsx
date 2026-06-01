import About from '@/components/About';
import HowItWorks from '@/components/HowItWorks';
import Ecosystem from '@/components/Ecosystem';
import CallToAction from '@/components/CallToAction';

export default function Home() {
  return (
    <main>
      <section className="py-20 px-4 md:px-8 bg-gradient-to-b from-blue-50 to-white dark:from-gray-900 dark:to-gray-800">
        <div className="max-w-4xl mx-auto text-center">
          <h1 className="text-4xl md:text-5xl font-bold mb-4 text-gray-900 dark:text-white">
            StellarVeriphy
          </h1>
          <p className="text-xl text-gray-600 dark:text-gray-300">
            Decentralized content verification and provenance on the Stellar blockchain.
          </p>
        </div>
      </section>
      <About />
      <HowItWorks />
      <Ecosystem />
      <CallToAction />
    </main>
  );
}
