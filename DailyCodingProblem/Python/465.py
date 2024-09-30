# This problem was asked by Google.
#
# Given the head of a singly linked list, reverse it in-place.


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __eq__(self, other):
        if not other:
            return False
        return self.val == other.val and self.next == other.next

    def to_list(self):
        result = []
        current = self
        while current:
            result.append(current.val)
            current = current.next
        return result


def create_linked_list(arr):
    if not arr:
        return None
    head = ListNode(arr[0])
    current = head
    for i in range(1, len(arr)):
        current.next = ListNode(arr[i])
        current = current.next
    return head


def reverse_linked_list(head: ListNode) -> ListNode:
    prev = None
    current = head
    while current:
        next_node = current.next
        current.next = prev
        prev = current
        current = next_node
    return prev


class TestReverseLinkedList:
    def test_example1(self):
        head = create_linked_list([1, 2, 3, 4, 5])
        reversed = reverse_linked_list(head)
        assert reversed.to_list() == [5, 4, 3, 2, 1]

    def test_empty_list(self):
        head = create_linked_list([])
        reversed = reverse_linked_list(head)
        assert reversed is None
