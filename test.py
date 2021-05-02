from logics import Play, Card
import unittest


class TestPlayComparing(unittest.TestCase):
    playPowerDic = {1: "single",
                    2: "pair",
                    3: "three of a kind",
                    4: "straight",
                    5: "flush",
                    6: "full house",
                    7: "straight flush",
                    8: "gang of four",
                    9: "gang of five",
                    10: "gang of six",
                    11: "gang of seven"
                    }

    def testPlayValue(self):
        self.assertEqual(Play([Card(3, 1), Card(2, 2), Card(1, 4), Card(3, 4), Card(3, 5)]).value(), 0)
        self.assertEqual(Play([Card(3, 1)]).value(), 1)
        self.assertEqual(Play([Card(3, 5), Card(2, 5)]).value(), 2)
        self.assertEqual(Play([Card(3, 5), Card(2, 5), Card(1, 5)]).value(), 3)
        self.assertEqual(Play([Card(4, 1), Card(2, 2), Card(3, 3), Card(1, 4), Card(2, 5)]).value(), 4)
        self.assertEqual(Play([Card(4, 1), Card(1, 1), Card(1, 2), Card(1, 5), Card(1, 8)]).value(), 5)
        self.assertEqual(Play([Card(1, 1), Card(1, 1), Card(2, 1), Card(2, 2), Card(3, 2)]).value(), 6)
        self.assertEqual(Play([Card(1, 1), Card(1, 1), Card(2, 2), Card(2, 2), Card(3, 2)]).value(), 6)
        self.assertEqual(Play([Card(3, 1), Card(3, 2), Card(3, 3), Card(3, 4), Card(3, 5)]).value(), 7)
        self.assertEqual(Play([Card(1, 1), Card(1, 1), Card(2, 1), Card(2, 1)]).value(), 8)
        self.assertEqual(Play([Card(1, 1), Card(1, 1), Card(2, 1), Card(2, 1), Card(3, 1)]).value(), 9)

    def testFullHouses(self):
        self.assertEqual(Play([Card(1, 4), Card(4, 4), Card(3, 7), Card(3, 7), Card(4, 7)]).value(), 6)
        self.assertEqual(Play([Card(2, 3), Card(3, 3), Card(4, 6), Card(3, 6), Card(2, 6)]).value(), 6)
        self.assertEqual(Play([Card(1, 4), Card(4, 4), Card(3, 7), Card(3, 7), Card(4, 7)]).value(), 6)
        self.assertEqual(Play([Card(2, 3), Card(3, 3), Card(4, 6), Card(3, 6), Card(2, 6)]).value(), 6)
        self.assertEqual(Play([Card(1, 5), Card(4, 5), Card(2, 5), Card(1, 2), Card(4, 2)]).value(), 6)
        self.assertEqual(Play([Card(4, 11), Card(4, 11), Card(4, 9), Card(3, 9), Card(3, 9)]).value(), 6)
        self.assertEqual(Play([Card(1, 5), Card(4, 5), Card(2, 5), Card(1, 2), Card(4, 2)]).value(), 6)
        self.assertEqual(Play([Card(4, 11), Card(4, 11), Card(4, 9), Card(3, 9), Card(3, 9)]).value(), 6)
        self.assertEqual(Play([Card(1, 10), Card(4, 10), Card(3, 10), Card(3, 1), Card(4, 1)]).value(), 6)
        self.assertEqual(Play([Card(1, 10), Card(4, 10), Card(4, 10), Card(3, 1), Card(2, 1)]).value(), 6)
        self.assertEqual(Play([Card(1, 10), Card(4, 10), Card(3, 10), Card(3, 1), Card(4, 1)]).value(), 6)
        self.assertEqual(Play([Card(1, 10), Card(4, 10), Card(4, 10), Card(3, 1), Card(2, 1)]).value(), 6)
        self.assertEqual(Play([Card(1, 1), Card(4, 1), Card(3, 1), Card(3, 7), Card(4, 7)]).value(), 6)
        self.assertEqual(Play([Card(2, 1), Card(3, 1), Card(4, 3), Card(3, 3), Card(2, 3)]).value(), 6)
        self.assertEqual(Play([Card(1, 1), Card(4, 1), Card(3, 1), Card(3, 7), Card(4, 7)]).value(), 6)
        self.assertEqual(Play([Card(2, 1), Card(3, 1), Card(4, 3), Card(3, 3), Card(2, 3)]).value(), 6)

    def testFullHouseComparison(self):
        self.assertEqual(Play([Card(1, 4), Card(4, 4), Card(3, 7), Card(3, 7), Card(4, 7)]) > Play(
            [Card(2, 3), Card(3, 3), Card(4, 6), Card(3, 6), Card(2, 6)]), True)
        self.assertEqual(Play([Card(1, 4), Card(4, 4), Card(3, 7), Card(3, 7), Card(4, 7)]) < Play(
            [Card(2, 3), Card(3, 3), Card(4, 6), Card(3, 6), Card(2, 6)]), False)
        self.assertEqual(Play([Card(1, 5), Card(4, 5), Card(2, 5), Card(1, 2), Card(4, 2)]) < Play(
            [Card(4, 11), Card(4, 11), Card(4, 9), Card(3, 9), Card(3, 9)]), True)
        self.assertEqual(Play([Card(1, 5), Card(4, 5), Card(2, 5), Card(1, 2), Card(4, 2)]) > Play(
            [Card(4, 11), Card(4, 11), Card(4, 9), Card(3, 9), Card(3, 9)]), False)
        self.assertEqual(Play([Card(1, 10), Card(4, 10), Card(3, 10), Card(3, 1), Card(4, 1)]) < Play(
            [Card(1, 10), Card(4, 10), Card(4, 10), Card(3, 1), Card(2, 1)]), True)
        self.assertEqual(Play([Card(1, 10), Card(4, 10), Card(3, 10), Card(3, 1), Card(4, 1)]) > Play(
            [Card(1, 10), Card(4, 10), Card(4, 10), Card(3, 1), Card(2, 1)]), False)
        self.assertEqual(Play([Card(1, 1), Card(4, 1), Card(3, 1), Card(3, 7), Card(4, 7)]) > Play(
            [Card(2, 1), Card(3, 1), Card(4, 3), Card(3, 3), Card(2, 3)]), True)
        self.assertEqual(Play([Card(1, 1), Card(4, 1), Card(3, 1), Card(3, 7), Card(4, 7)]) < Play(
            [Card(2, 1), Card(3, 1), Card(4, 3), Card(3, 3), Card(2, 3)]), False)

    def testEmptyTable(self):
        self.assertEqual(Play([]) < Play(
            [Card(2, 1), Card(3, 1), Card(4, 3), Card(3, 3), Card(2, 3)]), True)
        self.assertEqual(Play([]) > Play(
            [Card(2, 1), Card(3, 1), Card(4, 3), Card(3, 3), Card(2, 3)]), False)


if __name__ == '__main__':
    unittest.main()
