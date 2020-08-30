//
// Created by 苏胤榕 on 2020/8/30.
//

#ifndef CPP_LIST_IMPLEMENT_LIST_H
#define CPP_LIST_IMPLEMENT_LIST_H

typedef int Rank; //秩

#define ListNodePosi(T)  ListNode<T>*

template <typename T>
struct ListNode {
    //typedef  ListNode<T>* ListNodePosi;
    //using ListNodePosi = ListNode<T>*;

    //成员
    T data;
    ListNodePosi pred;
    ListNodePosi succ;

    // 构造函数
    ListNode() = default;
    explicit ListNode(T e, ListNodePosi(T) p = nullptr, ListNodePosi(T) s = nullptr)
        : data ( e ), pred ( p ), succ ( s ) {}

    //操作接口
    ListNodePosi(T) insertAsPred(T const& e); //当前节点之前插入节点
    ListNodePosi(T) insertasSucc(T const& e); //当前节点之后插入节点


//    // public
//
//    size();
//    first();
//    last();
//    insertAsFirst(e);
//    insertAdLast(e);
//    insertA(p, e);
//    insertB(p, e);
//    remove(p);
//    disordered();
//    sort();
//    find(e);
//    search(e);
//    deduplicate();
//    uniquify();
//    traverse();

};

//template <typename T>
//using ListNodePosi = ListNode<T>*;
//


#endif //CPP_LIST_IMPLEMENT_LIST_H
