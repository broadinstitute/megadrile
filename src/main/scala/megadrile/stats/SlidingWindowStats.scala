package megadrile.stats

import megadrile.genomics.Variant
import megadrile.stats.SlidingWindowStats.{VariantResult, VariantWithStats}

import scala.collection.mutable

final class SlidingWindowStats(windowSize: Int, consumer: VariantResult => Unit) {

  private val deque = new mutable.ArrayDeque[VariantWithStats]()

  private def outOfWindow(earlier: VariantWithStats, later: VariantWithStats): Boolean = {
    val earlierVariant = earlier.variant
    val laterVariant = later.variant
    (earlierVariant.chrom != laterVariant.chrom) || (earlierVariant.pos + windowSize < laterVariant.pos)
  }

  def addVariantStats(variant: Variant, variantStats: VariantStats): Unit = {
    deque.append(VariantWithStats(variant, variantStats))
    while(deque.nonEmpty && outOfWindow(deque.head, deque.last)) {
      flushOldest()
    }
  }

  private def flushOldest(): Unit = {
    val oldest = deque.removeHead()
    val ldsBuilder = Seq.newBuilder[Double]
    for(other <- deque) {
      if(!outOfWindow(oldest, other)) {
        ldsBuilder += oldest.variantStats.correlation(other.variantStats)
      }
    }
    val result = VariantResult(oldest.variant, oldest.variantStats.mean, ldsBuilder.result())
    consumer(result)
  }

  def flushAll(): Unit = {
    while(deque.nonEmpty) {
      flushOldest()
    }
  }
}

object SlidingWindowStats {
  final case class VariantWithStats(variant: Variant, variantStats: VariantStats)
  final case class VariantResult(variant: Variant, mean: Double, lds: Seq[Double])
}
