; ModuleID = '1y7fujq39nar5v9h'
source_filename = "1y7fujq39nar5v9h"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::fmt::Arguments" = type { { [0 x { [0 x i8]*, i64 }]*, i64 }, { i64*, i64 }, { [0 x { i8*, i64* }]*, i64 } }
%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }

@alloc2 = private unnamed_addr constant <{ [10 x i8] }> <{ [10 x i8] c"Rust in C\0A" }>, align 1
@alloc3 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [10 x i8] }>, <{ [10 x i8] }>* @alloc2, i32 0, i32 0, i32 0), [8 x i8] c"\0A\00\00\00\00\00\00\00" }>, align 8
@alloc10 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 8
@alloc7 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc8 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [12 x i8] }>, <{ [12 x i8] }>* @alloc7, i32 0, i32 0, i32 0), [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc11 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/fmt/mod.rs" }>, align 1
@alloc12 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [75 x i8] }>, <{ [75 x i8] }>* @alloc11, i32 0, i32 0, i32 0), [16 x i8] c"K\00\00\00\00\00\00\00\81\01\00\00\0D\00\00\00" }>, align 8
@__rustc_debug_gdb_scripts_section__ = linkonce_odr unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1

; Function Attrs: nonlazybind uwtable
define void @test_func() unnamed_addr #0 !dbg !23 {
start:
  call void @priv_func(), !dbg !29
  br label %bb1, !dbg !29

bb1:                                              ; preds = %start
  ret void, !dbg !30
}

; Function Attrs: nonlazybind uwtable
define void @priv_func() unnamed_addr #0 !dbg !31 {
start:
  %_2 = alloca %"core::fmt::Arguments", align 8
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hb15049fba945ba4dE(%"core::fmt::Arguments"* noalias nocapture noundef sret(%"core::fmt::Arguments") dereferenceable(48) %_2, [0 x { [0 x i8]*, i64 }]* noundef nonnull align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc3 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* noundef nonnull align 8 bitcast (<{ [0 x i8] }>* @alloc10 to [0 x { i8*, i64* }]*), i64 0), !dbg !32
  br label %bb1, !dbg !32

bb1:                                              ; preds = %start
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17hcbc8e5359e4501b6E(%"core::fmt::Arguments"* noalias nocapture noundef dereferenceable(48) %_2), !dbg !32
  br label %bb2, !dbg !32

bb2:                                              ; preds = %bb1
  ret void, !dbg !33
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117hb15049fba945ba4dE(%"core::fmt::Arguments"* noalias nocapture noundef sret(%"core::fmt::Arguments") dereferenceable(48) %0, [0 x { [0 x i8]*, i64 }]* noundef nonnull align 8 %pieces.0, i64 %pieces.1, [0 x { i8*, i64* }]* noundef nonnull align 8 %args.0, i64 %args.1) unnamed_addr #1 !dbg !35 {
start:
  %args.dbg.spill = alloca { [0 x { i8*, i64* }]*, i64 }, align 8
  %pieces.dbg.spill = alloca { [0 x { [0 x i8]*, i64 }]*, i64 }, align 8
  %_23 = alloca { i64*, i64 }, align 8
  %_15 = alloca %"core::fmt::Arguments", align 8
  %_3 = alloca i8, align 1
  %1 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces.dbg.spill, i32 0, i32 0
  store [0 x { [0 x i8]*, i64 }]* %pieces.0, [0 x { [0 x i8]*, i64 }]** %1, align 8
  %2 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces.dbg.spill, i32 0, i32 1
  store i64 %pieces.1, i64* %2, align 8
  call void @llvm.dbg.declare(metadata { [0 x { [0 x i8]*, i64 }]*, i64 }* %pieces.dbg.spill, metadata !149, metadata !DIExpression()), !dbg !151
  %3 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %args.dbg.spill, i32 0, i32 0
  store [0 x { i8*, i64* }]* %args.0, [0 x { i8*, i64* }]** %3, align 8
  %4 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %args.dbg.spill, i32 0, i32 1
  store i64 %args.1, i64* %4, align 8
  call void @llvm.dbg.declare(metadata { [0 x { i8*, i64* }]*, i64 }* %args.dbg.spill, metadata !150, metadata !DIExpression()), !dbg !152
  %_4 = icmp ult i64 %pieces.1, %args.1, !dbg !153
  br i1 %_4, label %bb1, label %bb2, !dbg !153

bb2:                                              ; preds = %start
  %_12 = add i64 %args.1, 1, !dbg !154
  %_9 = icmp ugt i64 %pieces.1, %_12, !dbg !155
  %5 = zext i1 %_9 to i8, !dbg !153
  store i8 %5, i8* %_3, align 1, !dbg !153
  br label %bb3, !dbg !153

bb1:                                              ; preds = %start
  store i8 1, i8* %_3, align 1, !dbg !153
  br label %bb3, !dbg !153

bb3:                                              ; preds = %bb2, %bb1
  %6 = load i8, i8* %_3, align 1, !dbg !153, !range !156
  %7 = trunc i8 %6 to i1, !dbg !153
  br i1 %7, label %bb4, label %bb6, !dbg !153

bb6:                                              ; preds = %bb3
  %8 = bitcast { i64*, i64 }* %_23 to {}**, !dbg !157
  store {}* null, {}** %8, align 8, !dbg !157
  %9 = bitcast %"core::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i64 }]*, i64 }*, !dbg !158
  %10 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %9, i32 0, i32 0, !dbg !158
  store [0 x { [0 x i8]*, i64 }]* %pieces.0, [0 x { [0 x i8]*, i64 }]** %10, align 8, !dbg !158
  %11 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %9, i32 0, i32 1, !dbg !158
  store i64 %pieces.1, i64* %11, align 8, !dbg !158
  %12 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 1, !dbg !158
  %13 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_23, i32 0, i32 0, !dbg !158
  %14 = load i64*, i64** %13, align 8, !dbg !158
  %15 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_23, i32 0, i32 1, !dbg !158
  %16 = load i64, i64* %15, align 8, !dbg !158
  %17 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %12, i32 0, i32 0, !dbg !158
  store i64* %14, i64** %17, align 8, !dbg !158
  %18 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %12, i32 0, i32 1, !dbg !158
  store i64 %16, i64* %18, align 8, !dbg !158
  %19 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 2, !dbg !158
  %20 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %19, i32 0, i32 0, !dbg !158
  store [0 x { i8*, i64* }]* %args.0, [0 x { i8*, i64* }]** %20, align 8, !dbg !158
  %21 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %19, i32 0, i32 1, !dbg !158
  store i64 %args.1, i64* %21, align 8, !dbg !158
  ret void, !dbg !159

bb4:                                              ; preds = %bb3
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hb15049fba945ba4dE(%"core::fmt::Arguments"* noalias nocapture noundef sret(%"core::fmt::Arguments") dereferenceable(48) %_15, [0 x { [0 x i8]*, i64 }]* noundef nonnull align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc8 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* noundef nonnull align 8 bitcast (<{ [0 x i8] }>* @alloc10 to [0 x { i8*, i64* }]*), i64 0), !dbg !160
  br label %bb5, !dbg !160

bb5:                                              ; preds = %bb4
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17he1bbc7336d49a357E(%"core::fmt::Arguments"* noalias nocapture noundef dereferenceable(48) %_15, %"core::panic::location::Location"* noundef align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc12 to %"core::panic::location::Location"*)) #4, !dbg !160
  unreachable, !dbg !160
}

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17hcbc8e5359e4501b6E(%"core::fmt::Arguments"* noalias nocapture noundef dereferenceable(48)) unnamed_addr #0

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking9panic_fmt17he1bbc7336d49a357E(%"core::fmt::Arguments"* noalias nocapture noundef dereferenceable(48), %"core::panic::location::Location"* noundef align 8 dereferenceable(24)) unnamed_addr #3

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #4 = { noreturn }

!llvm.module.flags = !{!0, !1, !2}
!llvm.dbg.cu = !{!3}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !4, producer: "clang LLVM (rustc version 1.60.0 (7737e0b5c 2022-04-04))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !5)
!4 = !DIFile(filename: "src/lib.rs/@/1y7fujq39nar5v9h", directory: "/home/guybl/code_files/c-rust/src/test-lib")
!5 = !{!6, !18}
!6 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !8, file: !7, baseType: !12, size: 8, align: 8, flags: DIFlagEnumClass, elements: !13)
!7 = !DIFile(filename: "<unknown>", directory: "")
!8 = !DINamespace(name: "v1", scope: !9)
!9 = !DINamespace(name: "rt", scope: !10)
!10 = !DINamespace(name: "fmt", scope: !11)
!11 = !DINamespace(name: "core", scope: null)
!12 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!13 = !{!14, !15, !16, !17}
!14 = !DIEnumerator(name: "Left", value: 0)
!15 = !DIEnumerator(name: "Right", value: 1)
!16 = !DIEnumerator(name: "Center", value: 2)
!17 = !DIEnumerator(name: "Unknown", value: 3)
!18 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Result", scope: !19, file: !7, baseType: !12, size: 8, align: 8, flags: DIFlagEnumClass, elements: !20)
!19 = !DINamespace(name: "result", scope: !11)
!20 = !{!21, !22}
!21 = !DIEnumerator(name: "Ok", value: 0)
!22 = !DIEnumerator(name: "Err", value: 1)
!23 = distinct !DISubprogram(name: "test_func", scope: !25, file: !24, line: 3, type: !26, scopeLine: 3, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !3, templateParams: !28, retainedNodes: !28)
!24 = !DIFile(filename: "src/lib.rs", directory: "/home/guybl/code_files/c-rust/src/test-lib", checksumkind: CSK_MD5, checksum: "d3598753a1832895e1397d55d69fbc2e")
!25 = !DINamespace(name: "test_lib", scope: null)
!26 = !DISubroutineType(types: !27)
!27 = !{null}
!28 = !{}
!29 = !DILocation(line: 4, column: 5, scope: !23)
!30 = !DILocation(line: 5, column: 2, scope: !23)
!31 = distinct !DISubprogram(name: "priv_func", scope: !25, file: !24, line: 8, type: !26, scopeLine: 8, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !3, templateParams: !28, retainedNodes: !28)
!32 = !DILocation(line: 9, column: 5, scope: !31)
!33 = !DILocation(line: 10, column: 2, scope: !34)
!34 = !DILexicalBlockFile(scope: !31, file: !24, discriminator: 0)
!35 = distinct !DISubprogram(name: "new_v1", linkageName: "_ZN4core3fmt9Arguments6new_v117hb15049fba945ba4dE", scope: !37, file: !36, line: 383, type: !146, scopeLine: 383, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !28, retainedNodes: !148)
!36 = !DIFile(filename: "/rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "7d486092cd3957d561caa81fafa9cc68")
!37 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !10, file: !7, size: 384, align: 64, elements: !38, templateParams: !28, identifier: "6a0136dc63736d7b1a45220ab0e42c36")
!38 = !{!39, !51, !100}
!39 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !37, file: !7, baseType: !40, size: 128, align: 64)
!40 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !7, size: 128, align: 64, elements: !41, templateParams: !28, identifier: "a673d89da0bdcf4d8c9ca87ae2c56d84")
!41 = !{!42, !50}
!42 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !40, file: !7, baseType: !43, size: 64, align: 64)
!43 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !44, size: 64, align: 64, dwarfAddressSpace: 0)
!44 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !7, size: 128, align: 64, elements: !45, templateParams: !28, identifier: "84eec819988617519061e0b609a72fe3")
!45 = !{!46, !48}
!46 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !44, file: !7, baseType: !47, size: 64, align: 64)
!47 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !12, size: 64, align: 64, dwarfAddressSpace: 0)
!48 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !44, file: !7, baseType: !49, size: 64, align: 64, offset: 64)
!49 = !DIBasicType(name: "usize", size: 64, encoding: DW_ATE_unsigned)
!50 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !40, file: !7, baseType: !49, size: 64, align: 64, offset: 64)
!51 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !37, file: !7, baseType: !52, size: 128, align: 64, offset: 128)
!52 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::v1::Argument]>", scope: !53, file: !7, size: 128, align: 64, elements: !54, identifier: "4e971e86b650c8681454e87a8b961ac1")
!53 = !DINamespace(name: "option", scope: !11)
!54 = !{!55}
!55 = !DICompositeType(tag: DW_TAG_variant_part, scope: !53, file: !7, size: 128, align: 64, elements: !56, templateParams: !59, identifier: "4e971e86b650c8681454e87a8b961ac1_variant_part", discriminator: !99)
!56 = !{!57, !95}
!57 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !55, file: !7, baseType: !58, size: 128, align: 64, extraData: i64 0)
!58 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !52, file: !7, size: 128, align: 64, elements: !28, templateParams: !59, identifier: "4e971e86b650c8681454e87a8b961ac1::None")
!59 = !{!60}
!60 = !DITemplateTypeParameter(name: "T", type: !61)
!61 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::v1::Argument]", file: !7, size: 128, align: 64, elements: !62, templateParams: !28, identifier: "be7e57e20d0a21e8f8d215a1721634a")
!62 = !{!63, !94}
!63 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !61, file: !7, baseType: !64, size: 64, align: 64)
!64 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !65, size: 64, align: 64, dwarfAddressSpace: 0)
!65 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !8, file: !7, size: 448, align: 64, elements: !66, templateParams: !28, identifier: "3df133947dab7c728cd6a9be5ca16516")
!66 = !{!67, !68}
!67 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !65, file: !7, baseType: !49, size: 64, align: 64)
!68 = !DIDerivedType(tag: DW_TAG_member, name: "format", scope: !65, file: !7, baseType: !69, size: 384, align: 64, offset: 64)
!69 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormatSpec", scope: !8, file: !7, size: 384, align: 64, elements: !70, templateParams: !28, identifier: "e8023547c76f296ebe1b327e632da055")
!70 = !{!71, !73, !74, !76, !93}
!71 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !69, file: !7, baseType: !72, size: 32, align: 32, offset: 256)
!72 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_unsigned_char)
!73 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !69, file: !7, baseType: !6, size: 8, align: 8, offset: 320)
!74 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !69, file: !7, baseType: !75, size: 32, align: 32, offset: 288)
!75 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!76 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !69, file: !7, baseType: !77, size: 128, align: 64)
!77 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !8, file: !7, size: 128, align: 64, elements: !78, identifier: "180e6da4994dfae83738c6a3882cd8af")
!78 = !{!79}
!79 = !DICompositeType(tag: DW_TAG_variant_part, scope: !8, file: !7, size: 128, align: 64, elements: !80, templateParams: !28, identifier: "180e6da4994dfae83738c6a3882cd8af_variant_part", discriminator: !91)
!80 = !{!81, !85, !89}
!81 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !79, file: !7, baseType: !82, size: 128, align: 64, extraData: i64 0)
!82 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !77, file: !7, size: 128, align: 64, elements: !83, templateParams: !28, identifier: "180e6da4994dfae83738c6a3882cd8af::Is")
!83 = !{!84}
!84 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !82, file: !7, baseType: !49, size: 64, align: 64, offset: 64)
!85 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !79, file: !7, baseType: !86, size: 128, align: 64, extraData: i64 1)
!86 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !77, file: !7, size: 128, align: 64, elements: !87, templateParams: !28, identifier: "180e6da4994dfae83738c6a3882cd8af::Param")
!87 = !{!88}
!88 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !86, file: !7, baseType: !49, size: 64, align: 64, offset: 64)
!89 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !79, file: !7, baseType: !90, size: 128, align: 64, extraData: i64 2)
!90 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !77, file: !7, size: 128, align: 64, elements: !28, templateParams: !28, identifier: "180e6da4994dfae83738c6a3882cd8af::Implied")
!91 = !DIDerivedType(tag: DW_TAG_member, scope: !8, file: !7, baseType: !92, size: 64, align: 64, flags: DIFlagArtificial)
!92 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!93 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !69, file: !7, baseType: !77, size: 128, align: 64, offset: 128)
!94 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !61, file: !7, baseType: !49, size: 64, align: 64, offset: 64)
!95 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !55, file: !7, baseType: !96, size: 128, align: 64)
!96 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !52, file: !7, size: 128, align: 64, elements: !97, templateParams: !59, identifier: "4e971e86b650c8681454e87a8b961ac1::Some")
!97 = !{!98}
!98 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !96, file: !7, baseType: !61, size: 128, align: 64)
!99 = !DIDerivedType(tag: DW_TAG_member, scope: !53, file: !7, baseType: !92, size: 64, align: 64, flags: DIFlagArtificial)
!100 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !37, file: !7, baseType: !101, size: 128, align: 64, offset: 256)
!101 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::ArgumentV1]", file: !7, size: 128, align: 64, elements: !102, templateParams: !28, identifier: "30dc1df3c04522d1ef75dbb38d233817")
!102 = !{!103, !145}
!103 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !101, file: !7, baseType: !104, size: 64, align: 64)
!104 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !105, size: 64, align: 64, dwarfAddressSpace: 0)
!105 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentV1", scope: !10, file: !7, size: 128, align: 64, elements: !106, templateParams: !28, identifier: "2977922729a729ec883335acf354d97d")
!106 = !{!107, !110}
!107 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !105, file: !7, baseType: !108, size: 64, align: 64)
!108 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::{extern#0}::Opaque", baseType: !109, size: 64, align: 64, dwarfAddressSpace: 0)
!109 = !DICompositeType(tag: DW_TAG_structure_type, name: "Opaque", file: !7, align: 8, elements: !28, identifier: "b0c334d23a9c52ed8d1130e51e6a8b3c")
!110 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !105, file: !7, baseType: !111, size: 64, align: 64, offset: 64)
!111 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&core::fmt::{extern#0}::Opaque, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !112, size: 64, align: 64, dwarfAddressSpace: 0)
!112 = !DISubroutineType(types: !113)
!113 = !{!18, !108, !114}
!114 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !115, size: 64, align: 64, dwarfAddressSpace: 0)
!115 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !10, file: !7, size: 512, align: 64, elements: !116, templateParams: !28, identifier: "52efd35c2d51dd13809e2f52575c9238")
!116 = !{!117, !118, !119, !120, !133, !134}
!117 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !115, file: !7, baseType: !75, size: 32, align: 32, offset: 384)
!118 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !115, file: !7, baseType: !72, size: 32, align: 32, offset: 416)
!119 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !115, file: !7, baseType: !6, size: 8, align: 8, offset: 448)
!120 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !115, file: !7, baseType: !121, size: 128, align: 64)
!121 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !53, file: !7, size: 128, align: 64, elements: !122, identifier: "791bd15c3f8c7ea9dc6da9cb98dd3e54")
!122 = !{!123}
!123 = !DICompositeType(tag: DW_TAG_variant_part, scope: !53, file: !7, size: 128, align: 64, elements: !124, templateParams: !127, identifier: "791bd15c3f8c7ea9dc6da9cb98dd3e54_variant_part", discriminator: !99)
!124 = !{!125, !129}
!125 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !123, file: !7, baseType: !126, size: 128, align: 64, extraData: i64 0)
!126 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !121, file: !7, size: 128, align: 64, elements: !28, templateParams: !127, identifier: "791bd15c3f8c7ea9dc6da9cb98dd3e54::None")
!127 = !{!128}
!128 = !DITemplateTypeParameter(name: "T", type: !49)
!129 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !123, file: !7, baseType: !130, size: 128, align: 64, extraData: i64 1)
!130 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !121, file: !7, size: 128, align: 64, elements: !131, templateParams: !127, identifier: "791bd15c3f8c7ea9dc6da9cb98dd3e54::Some")
!131 = !{!132}
!132 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !130, file: !7, baseType: !49, size: 64, align: 64, offset: 64)
!133 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !115, file: !7, baseType: !121, size: 128, align: 64, offset: 128)
!134 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !115, file: !7, baseType: !135, size: 128, align: 64, offset: 256)
!135 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !7, size: 128, align: 64, elements: !136, templateParams: !28, identifier: "dcf491c120088d342d73bb0d3b5c01eb")
!136 = !{!137, !140}
!137 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !135, file: !7, baseType: !138, size: 64, align: 64)
!138 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !139, size: 64, align: 64, dwarfAddressSpace: 0)
!139 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !7, align: 8, elements: !28, templateParams: !28, identifier: "8b6438dc5b95edbc7265d3c4eb2c6b32")
!140 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !135, file: !7, baseType: !141, size: 64, align: 64, offset: 64)
!141 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 3]", baseType: !142, size: 64, align: 64, dwarfAddressSpace: 0)
!142 = !DICompositeType(tag: DW_TAG_array_type, baseType: !49, size: 192, align: 64, elements: !143)
!143 = !{!144}
!144 = !DISubrange(count: 3, lowerBound: 0)
!145 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !101, file: !7, baseType: !49, size: 64, align: 64, offset: 64)
!146 = !DISubroutineType(types: !147)
!147 = !{!37, !40, !101}
!148 = !{!149, !150}
!149 = !DILocalVariable(name: "pieces", arg: 1, scope: !35, file: !36, line: 383, type: !40)
!150 = !DILocalVariable(name: "args", arg: 2, scope: !35, file: !36, line: 383, type: !101)
!151 = !DILocation(line: 383, column: 25, scope: !35)
!152 = !DILocation(line: 383, column: 53, scope: !35)
!153 = !DILocation(line: 384, column: 12, scope: !35)
!154 = !DILocation(line: 384, column: 56, scope: !35)
!155 = !DILocation(line: 384, column: 41, scope: !35)
!156 = !{i8 0, i8 2}
!157 = !DILocation(line: 387, column: 34, scope: !35)
!158 = !DILocation(line: 387, column: 9, scope: !35)
!159 = !DILocation(line: 388, column: 6, scope: !35)
!160 = !DILocation(line: 385, column: 13, scope: !35)
