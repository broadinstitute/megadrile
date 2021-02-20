package megadrile.stats

import megadrile.genomics.Variant
import megadrile.stats.SlidingWindowStats.{Consumer, VariantResult, VariantWithStats}

import scala.collection.mutable

final class SlidingWindowStats(windowSize: Int, consumer: Consumer) {

  private val deque = new mutable.ArrayDeque[VariantWithStats]()

  var nVariants: Long = 0L

  private def outOfWindow(earlier: VariantWithStats, later: VariantWithStats): Boolean = {
    val earlierVariant = earlier.variant
    val laterVariant = later.variant
    (earlierVariant.chrom != laterVariant.chrom) || (earlierVariant.pos + windowSize < laterVariant.pos)
  }

  def addVariantStats(variant: Variant, variantStats: VariantStats): Unit = {
    deque.append(VariantWithStats(variant, variantStats))
    while (deque.nonEmpty && outOfWindow(deque.head, deque.last)) {
      flushEarliest()
    }
    nVariants += 1L
  }

  private def flushEarliest(): Unit = {
    val earliest = deque.removeHead()
    val ldsBuilder = Seq.newBuilder[Double]
    consumer.nextVariant(earliest.variant)
    consumer.setMean(earliest.variantStats.mean)
    for (other <- deque) {
      if (!outOfWindow(earliest, other)) {
        consumer.addCorrelation(earliest.variantStats.correlation(other.variantStats))
      }
    }
    consumer.doneWithVariant()
  }

  def flushAll(): Unit = {
    while (deque.nonEmpty) {
      flushEarliest()
    }
  }
}

object SlidingWindowStats {

  trait Consumer {
    def nextVariant(variant: Variant): Unit

    def setMean(mean: Double): Unit

    def addCorrelation(correlation: Double): Unit

    def doneWithVariant(): Unit
  }

  object Consumer {

    class PrintConsumer(printer: String => Unit) extends Consumer {
      override def nextVariant(variant: Variant): Unit = printer(variant.toString + "\t")

      override def setMean(mean: Double): Unit = printer(mean + "\t")

      override def addCorrelation(correlation: Double): Unit = printer(correlation + "\t")

      override def doneWithVariant(): Unit = printer("\n")
    }

    object NoOpConsumer extends Consumer {
      override def nextVariant(variant: Variant): Unit = ()

      override def setMean(mean: Double): Unit = ()

      override def addCorrelation(correlation: Double): Unit = ()

      override def doneWithVariant(): Unit = ()
    }

  }

  final case class VariantWithStats(variant: Variant, variantStats: VariantStats)

  final case class VariantResult(variant: Variant, mean: Double, lds: Seq[Double])

}
