; ModuleID = 'probe1.3a1fbbbh-cgu.0'
source_filename = "probe1.3a1fbbbh-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"core::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"core::fmt::::Opaque" = type {}
%"core::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i64] }
%"alloc::string::String" = type { [0 x i64], %"alloc::vec::Vec<u8>", [0 x i64] }
%"alloc::vec::Vec<u8>" = type { [0 x i64], { i8*, i64 }, [0 x i64], i64, [0 x i64] }
%"core::ptr::Repr<u8>" = type { [2 x i64] }
%"core::marker::PhantomData<u8>" = type {}
%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>" = type { [0 x i64], {}*, [2 x i64] }
%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some" = type { [0 x i64], { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }, [0 x i64] }
%"alloc::alloc::Global" = type {}

@alloc1 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 1
@alloc2 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [0 x i8] }>, <{ [0 x i8] }>* @alloc1, i32 0, i32 0, i32 0), [8 x i8] zeroinitializer }>, align 8
@0 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* bitcast (<{ i8*, [8 x i8] }>* @alloc2 to i8*), [0 x i8] zeroinitializer }>, align 8
@alloc6 = private unnamed_addr constant <{ [8 x i8] }> zeroinitializer, align 8
@1 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @alloc6, i32 0, i32 0, i32 0), [0 x i8] zeroinitializer }>, align 8

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h3ac9c8ec2972d05dE"(i8* nonnull %unique) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_2 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h954724e671dc2d2cE"(i8* nonnull %unique)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h6c2b4bcd860bcb17E"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::fmt::ArgumentV1::new
; Function Attrs: uwtable
define { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h3a2356564b0426d9E(i64* noalias readonly align 8 dereferenceable(8) %x, i1 (i64*, %"core::fmt::Formatter"*)* nonnull %f) unnamed_addr #1 {
start:
  %0 = alloca %"core::fmt::::Opaque"*, align 8
  %1 = alloca i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, align 8
  %2 = alloca { i8*, i8* }, align 8
  %3 = bitcast i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1 to i1 (i64*, %"core::fmt::Formatter"*)**
  store i1 (i64*, %"core::fmt::Formatter"*)* %f, i1 (i64*, %"core::fmt::Formatter"*)** %3, align 8
  %_3 = load i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1, align 8, !nonnull !1
  br label %bb1

bb1:                                              ; preds = %start
  %4 = bitcast %"core::fmt::::Opaque"** %0 to i64**
  store i64* %x, i64** %4, align 8
  %_5 = load %"core::fmt::::Opaque"*, %"core::fmt::::Opaque"** %0, align 8, !nonnull !1
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8*, i8* }* %2 to %"core::fmt::::Opaque"**
  store %"core::fmt::::Opaque"* %_5, %"core::fmt::::Opaque"** %5, align 8
  %6 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %7 = bitcast i8** %6 to i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)**
  store i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)* %_3, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %7, align 8
  %8 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 0
  %9 = load i8*, i8** %8, align 8, !nonnull !1
  %10 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %11 = load i8*, i8** %10, align 8, !nonnull !1
  %12 = insertvalue { i8*, i8* } undef, i8* %9, 0
  %13 = insertvalue { i8*, i8* } %12, i8* %11, 1
  ret { i8*, i8* } %13
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h90a6baa7ad974f20E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %0, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 %pieces.0, i64 %pieces.1, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %args.0, i64 %args.1) unnamed_addr #0 {
start:
  %_4 = alloca { i64*, i64 }, align 8
  %1 = bitcast { i64*, i64 }* %_4 to {}**
  store {}* null, {}** %1, align 8
  %2 = bitcast %"core::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i64 }]*, i64 }*
  %3 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %2, i32 0, i32 0
  store [0 x { [0 x i8]*, i64 }]* %pieces.0, [0 x { [0 x i8]*, i64 }]** %3, align 8
  %4 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %2, i32 0, i32 1
  store i64 %pieces.1, i64* %4, align 8
  %5 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 3
  %6 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 0
  %7 = load i64*, i64** %6, align 8
  %8 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 1
  %9 = load i64, i64* %8, align 8
  %10 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %5, i32 0, i32 0
  store i64* %7, i64** %10, align 8
  %11 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %5, i32 0, i32 1
  store i64 %9, i64* %11, align 8
  %12 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 5
  %13 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %12, i32 0, i32 0
  store [0 x { i8*, i8* }]* %args.0, [0 x { i8*, i8* }]** %13, align 8
  %14 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %12, i32 0, i32 1
  store i64 %args.1, i64* %14, align 8
  ret void
}

; core::num::NonZeroUsize::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize13new_unchecked17h86b92266c2b76e35E(i64 %n) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 %n, i64* %0, align 8
  %1 = load i64, i64* %0, align 8, !range !2
  ret i64 %1
}

; core::num::NonZeroUsize::get
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize3get17h337a535a9887c3a8E(i64 %self) unnamed_addr #0 {
start:
  ret i64 %self
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define void @_ZN4core3ptr13drop_in_place17h79bb10cac534cac2E(%"alloc::string::String"* %_1) unnamed_addr #1 {
start:
  %0 = alloca {}, align 1
  %1 = bitcast %"alloc::string::String"* %_1 to %"alloc::vec::Vec<u8>"*
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17hf1a0c6699125cc23E(%"alloc::vec::Vec<u8>"* %1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define void @_ZN4core3ptr13drop_in_place17ha2a03b6a656f6e1bE({ i8*, i64 }* %_1) unnamed_addr #1 {
start:
  %0 = alloca {}, align 1
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbfb901143d0e3d77E"({ i8*, i64 }* align 8 dereferenceable(16) %_1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define void @_ZN4core3ptr13drop_in_place17hf1a0c6699125cc23E(%"alloc::vec::Vec<u8>"* %_1) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %0 = alloca {}, align 1
; invoke <alloc::vec::Vec<T> as core::ops::drop::Drop>::drop
  invoke void @"_ZN66_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h78fd3523767da90eE"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %_1)
          to label %bb4 unwind label %funclet_bb3

bb1:                                              ; preds = %bb3
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %bb4
  ret void

bb3:                                              ; preds = %funclet_bb3
  %1 = bitcast %"alloc::vec::Vec<u8>"* %_1 to { i8*, i64 }*
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17ha2a03b6a656f6e1bE({ i8*, i64 }* %1) #6 [ "funclet"(token %cleanuppad) ]
  br label %bb1

bb4:                                              ; preds = %start
  %2 = bitcast %"alloc::vec::Vec<u8>"* %_1 to { i8*, i64 }*
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17ha2a03b6a656f6e1bE({ i8*, i64 }* %2)
  br label %bb2

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3
}

; core::ptr::slice_from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17hab08c684831e1a3dE(i8* %data, i64 %len) unnamed_addr #0 {
start:
  %_4 = alloca { i8*, i64 }, align 8
  %_3 = alloca %"core::ptr::Repr<u8>", align 8
  %0 = bitcast { i8*, i64 }* %_4 to i8**
  store i8* %data, i8** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  store i64 %len, i64* %1, align 8
  %2 = bitcast %"core::ptr::Repr<u8>"* %_3 to { i8*, i64 }*
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = bitcast %"core::ptr::Repr<u8>"* %_3 to { [0 x i8]*, i64 }*
  %10 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 0
  %11 = load [0 x i8]*, [0 x i8]** %10, align 8
  %12 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %11, 0
  %15 = insertvalue { [0 x i8]*, i64 } %14, i64 %13, 1
  ret { [0 x i8]*, i64 } %15
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h03edcfaaf6d25856E"(i8* %ptr) unnamed_addr #0 {
start:
  %_5 = alloca %"core::marker::PhantomData<u8>", align 1
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = bitcast i8** %0 to %"core::marker::PhantomData<u8>"*
  %2 = load i8*, i8** %0, align 8, !nonnull !1
  ret i8* %2
}

; core::ptr::unique::Unique<T>::cast
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17hea9723b381c13df4E"(i8* nonnull %self) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_3 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h954724e671dc2d2cE"(i8* nonnull %self)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::unique::Unique<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h03edcfaaf6d25856E"(i8* %_3)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h954724e671dc2d2cE"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::ptr::mut_ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h91bee98b3183ec41E"(i8* %self) unnamed_addr #0 {
start:
  br label %bb1

bb1:                                              ; preds = %start
  %0 = icmp eq i8* %self, null
  ret i1 %0
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h6c2b4bcd860bcb17E"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = load i8*, i8** %0, align 8, !nonnull !1
  ret i8* %1
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17ha92c35c119074284E"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::alloc::layout::Layout::from_size_align_unchecked
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h44f611829a6ec18bE(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %0 = alloca { i64, i64 }, align 8
; call core::num::NonZeroUsize::new_unchecked
  %_4 = call i64 @_ZN4core3num12NonZeroUsize13new_unchecked17h86b92266c2b76e35E(i64 %align), !range !2
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast { i64, i64 }* %0 to i64*
  store i64 %size, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_4, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %6 = load i64, i64* %5, align 8, !range !2
  %7 = insertvalue { i64, i64 } undef, i64 %4, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; core::alloc::layout::Layout::size
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout4size17h60452e37a6dd595cE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64, i64 }* %self to i64*
  %1 = load i64, i64* %0, align 8
  ret i64 %1
}

; core::alloc::layout::Layout::align
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout5align17hd6717a358b4fdaeeE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %_2 = load i64, i64* %0, align 8, !range !2
; call core::num::NonZeroUsize::get
  %1 = call i64 @_ZN4core3num12NonZeroUsize3get17h337a535a9887c3a8E(i64 %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; <T as core::convert::Into<U>>::into
; Function Attrs: uwtable
define nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h41c11f782bce65dcE"(i8* nonnull %self) unnamed_addr #1 {
start:
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %0 = call nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h3ac9c8ec2972d05dE"(i8* nonnull %self)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; alloc::vec::Vec<T>::as_mut_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN5alloc3vec12Vec$LT$T$GT$10as_mut_ptr17hf13a47a81ea9c23aE"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_2 = bitcast %"alloc::vec::Vec<u8>"* %self to { i8*, i64 }*
; call alloc::raw_vec::RawVec<T,A>::ptr
  %ptr = call i8* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h7c835acbdace094dE"({ i8*, i64 }* noalias readonly align 8 dereferenceable(16) %_2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_5 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h91bee98b3183ec41E"(i8* %ptr)
  br label %bb2

bb2:                                              ; preds = %bb1
  %_4 = xor i1 %_5, true
  call void @llvm.assume(i1 %_4)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %ptr
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc5alloc7dealloc17h6bb033c0a86cd353E(i8* %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17h60452e37a6dd595cE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_6 = call i64 @_ZN4core5alloc6layout6Layout5align17hd6717a358b4fdaeeE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  call void @__rust_dealloc(i8* %ptr, i64 %_4, i64 %_6)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: uwtable
define void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h4198838aa4f72a96E"(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* noalias nocapture sret dereferenceable(24) %0, { i8*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %1 = alloca i64, align 8
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %_14 = alloca { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }, align 8
  %_2 = alloca i8, align 1
  store i64 1, i64* %3, align 8
  %4 = load i64, i64* %3, align 8
  br label %bb5

bb1:                                              ; preds = %bb3, %bb5
  store i8 1, i8* %_2, align 1
  br label %bb4

bb2:                                              ; preds = %bb3
  store i8 0, i8* %_2, align 1
  br label %bb4

bb3:                                              ; preds = %bb5
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %_6 = load i64, i64* %5, align 8
  %_5 = icmp eq i64 %_6, 0
  br i1 %_5, label %bb1, label %bb2

bb4:                                              ; preds = %bb2, %bb1
  %6 = load i8, i8* %_2, align 1, !range !3
  %7 = trunc i8 %6 to i1
  br i1 %7, label %bb7, label %bb6

bb5:                                              ; preds = %start
  %_3 = icmp eq i64 %4, 0
  br i1 %_3, label %bb1, label %bb3

bb6:                                              ; preds = %bb4
  store i64 1, i64* %2, align 8
  %8 = load i64, i64* %2, align 8
  br label %bb8

bb7:                                              ; preds = %bb4
  %9 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %0 to {}**
  store {}* null, {}** %9, align 8
  br label %bb13

bb8:                                              ; preds = %bb6
  store i64 1, i64* %1, align 8
  %10 = load i64, i64* %1, align 8
  br label %bb9

bb9:                                              ; preds = %bb8
  %11 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %_10 = load i64, i64* %11, align 8
  %size = mul i64 %10, %_10
; call core::alloc::layout::Layout::from_size_align_unchecked
  %12 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h44f611829a6ec18bE(i64 %size, i64 %8)
  %layout.0 = extractvalue { i64, i64 } %12, 0
  %layout.1 = extractvalue { i64, i64 } %12, 1
  br label %bb10

bb10:                                             ; preds = %bb9
  %13 = bitcast { i8*, i64 }* %self to i8**
  %_17 = load i8*, i8** %13, align 8, !nonnull !1
; call core::ptr::unique::Unique<T>::cast
  %_16 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17hea9723b381c13df4E"(i8* nonnull %_17)
  br label %bb11

bb11:                                             ; preds = %bb10
; call <T as core::convert::Into<U>>::into
  %_15 = call nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h41c11f782bce65dcE"(i8* nonnull %_16)
  br label %bb12

bb12:                                             ; preds = %bb11
  %14 = bitcast { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %_14 to i8**
  store i8* %_15, i8** %14, align 8
  %15 = getelementptr inbounds { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }, { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %_14, i32 0, i32 3
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %15, i32 0, i32 0
  store i64 %layout.0, i64* %16, align 8
  %17 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %15, i32 0, i32 1
  store i64 %layout.1, i64* %17, align 8
  %18 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %0 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %19 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %18 to { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }*
  %20 = bitcast { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %19 to i8*
  %21 = bitcast { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %_14 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %20, i8* align 8 %21, i64 24, i1 false)
  br label %bb13

bb13:                                             ; preds = %bb12, %bb7
  ret void
}

; alloc::raw_vec::RawVec<T,A>::ptr
; Function Attrs: uwtable
define i8* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h7c835acbdace094dE"({ i8*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %0 = bitcast { i8*, i64 }* %self to i8**
  %_2 = load i8*, i8** %0, align 8, !nonnull !1
; call core::ptr::unique::Unique<T>::as_ptr
  %1 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h954724e671dc2d2cE"(i8* nonnull %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %1
}

; <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
; Function Attrs: inlinehint uwtable
define internal void @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17he11482efa3a4e9f9E"(%"alloc::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %2 = alloca {}, align 1
  %layout = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %4, align 8
; call core::alloc::layout::Layout::size
  %_5 = call i64 @_ZN4core5alloc6layout6Layout4size17h60452e37a6dd595cE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
  %_4 = icmp ne i64 %_5, 0
  br i1 %_4, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  br label %bb6

bb3:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_7 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17ha92c35c119074284E"(i8* nonnull %ptr)
  br label %bb4

bb4:                                              ; preds = %bb3
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_9.0 = load i64, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_9.1 = load i64, i64* %6, align 8, !range !2
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17h6bb033c0a86cd353E(i8* %_7, i64 %_9.0, i64 %_9.1)
  br label %bb5

bb5:                                              ; preds = %bb4
  br label %bb6

bb6:                                              ; preds = %bb2, %bb5
  ret void
}

; <alloc::vec::Vec<T> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @"_ZN66_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h78fd3523767da90eE"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self) unnamed_addr #1 {
start:
; call alloc::vec::Vec<T>::as_mut_ptr
  %_3 = call i8* @"_ZN5alloc3vec12Vec$LT$T$GT$10as_mut_ptr17hf13a47a81ea9c23aE"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self)
  br label %bb1

bb1:                                              ; preds = %start
  %0 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %self, i32 0, i32 3
  %_5 = load i64, i64* %0, align 8
; call core::ptr::slice_from_raw_parts_mut
  %1 = call { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17hab08c684831e1a3dE(i8* %_3, i64 %_5)
  %_2.0 = extractvalue { [0 x i8]*, i64 } %1, 0
  %_2.1 = extractvalue { [0 x i8]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbfb901143d0e3d77E"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %_2 = alloca %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", align 8
  %0 = alloca {}, align 1
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h4198838aa4f72a96E"(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* noalias nocapture sret dereferenceable(24) %_2, { i8*, i64 }* noalias readonly align 8 dereferenceable(16) %self)
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to {}**
  %2 = load {}*, {}** %1, align 8
  %3 = icmp ule {}* %2, null
  %_4 = select i1 %3, i64 0, i64 1
  %4 = icmp eq i64 %_4, 1
  br i1 %4, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  br label %bb5

bb3:                                              ; preds = %bb1
  %5 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %6 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %5 to { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }*
  %7 = bitcast { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %6 to i8**
  %ptr = load i8*, i8** %7, align 8, !nonnull !1
  %8 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %9 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %8 to { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }*
  %10 = getelementptr inbounds { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }, { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %9, i32 0, i32 3
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 0
  %layout.0 = load i64, i64* %11, align 8
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %10, i32 0, i32 1
  %layout.1 = load i64, i64* %12, align 8, !range !2
  %_7 = bitcast { i8*, i64 }* %self to %"alloc::alloc::Global"*
; call <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
  call void @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17he11482efa3a4e9f9E"(%"alloc::alloc::Global"* nonnull align 1 %_7, i8* nonnull %ptr, i64 %layout.0, i64 %layout.1)
  br label %bb4

bb4:                                              ; preds = %bb3
  br label %bb5

bb5:                                              ; preds = %bb4, %bb2
  ret void
}

; probe1::probe
; Function Attrs: uwtable
define void @_ZN6probe15probe17he203a3b61d9abeacE() unnamed_addr #1 {
start:
  %_11 = alloca i64*, align 8
  %_10 = alloca [1 x { i8*, i8* }], align 8
  %_3 = alloca %"core::fmt::Arguments", align 8
  %res = alloca %"alloc::string::String", align 8
  %_1 = alloca %"alloc::string::String", align 8
  %_18 = load [1 x { [0 x i8]*, i64 }]*, [1 x { [0 x i8]*, i64 }]** bitcast (<{ i8*, [0 x i8] }>* @0 to [1 x { [0 x i8]*, i64 }]**), align 8, !nonnull !1
  %_4.0 = bitcast [1 x { [0 x i8]*, i64 }]* %_18 to [0 x { [0 x i8]*, i64 }]*
  %_17 = load i64*, i64** bitcast (<{ i8*, [0 x i8] }>* @1 to i64**), align 8, !nonnull !1
  store i64* %_17, i64** %_11, align 8
  %arg0 = load i64*, i64** %_11, align 8, !nonnull !1
; call core::fmt::ArgumentV1::new
  %0 = call { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h3a2356564b0426d9E(i64* noalias readonly align 8 dereferenceable(8) %arg0, i1 (i64*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17h888c581d3a43bb3eE")
  %_14.0 = extractvalue { i8*, i8* } %0, 0
  %_14.1 = extractvalue { i8*, i8* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast [1 x { i8*, i8* }]* %_10 to { i8*, i8* }*
  %2 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %1, i32 0, i32 0
  store i8* %_14.0, i8** %2, align 8
  %3 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %1, i32 0, i32 1
  store i8* %_14.1, i8** %3, align 8
  %_7.0 = bitcast [1 x { i8*, i8* }]* %_10 to [0 x { i8*, i8* }]*
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h90a6baa7ad974f20E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_3, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 %_4.0, i64 1, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %_7.0, i64 1)
  br label %bb2

bb2:                                              ; preds = %bb1
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17h7df8a09d5933fde7E(%"alloc::string::String"* noalias nocapture sret dereferenceable(24) %res, %"core::fmt::Arguments"* noalias nocapture dereferenceable(48) %_3)
  br label %bb3

bb3:                                              ; preds = %bb2
  %4 = bitcast %"alloc::string::String"* %_1 to i8*
  %5 = bitcast %"alloc::string::String"* %res to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %4, i8* align 8 %5, i64 24, i1 false)
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17h79bb10cac534cac2E(%"alloc::string::String"* %_1)
  br label %bb4

bb4:                                              ; preds = %bb3
  ret void
}

declare i32 @__CxxFrameHandler3(...) unnamed_addr #2

; Function Attrs: nounwind willreturn
declare void @llvm.assume(i1) #3

; Function Attrs: nounwind uwtable
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #4

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #5

; core::fmt::num::imp::<impl core::fmt::LowerExp for isize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17h888c581d3a43bb3eE"(i64* noalias readonly align 8 dereferenceable(8), %"core::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #1

; alloc::fmt::format
; Function Attrs: uwtable
declare void @_ZN5alloc3fmt6format17h7df8a09d5933fde7E(%"alloc::string::String"* noalias nocapture sret dereferenceable(24), %"core::fmt::Arguments"* noalias nocapture dereferenceable(48)) unnamed_addr #1

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { "target-cpu"="x86-64" }
attributes #3 = { nounwind willreturn }
attributes #4 = { nounwind uwtable "target-cpu"="x86-64" }
attributes #5 = { argmemonly nounwind willreturn }
attributes #6 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{}
!2 = !{i64 1, i64 0}
!3 = !{i8 0, i8 2}
