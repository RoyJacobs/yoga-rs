#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// API created by bindgen
#![allow(dead_code)]
mod internal {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::convert::From;

pub enum FlexStyle {
	AlignItems(Align),
	AlignSelf(Align),
	BorderBottomWidth(f32),
	BorderLeftWidth(f32),
	BorderRightWidth(f32),
	BorderTopWidth(f32),
	BorderWidth(f32),
	Bottom(f32),
	Flex(f32),
	FlexDirection(FlexDirection),
	FlexWrap(Wrap),
	Height(f32),
	JustifyContent(Justify),
	Left(f32),
	Margin(f32),
	MarginBottom(f32),
	MarginHorizontal(f32),
	MarginLeft(f32),
	MarginRight(f32),
	MarginTop(f32),
	MarginVertical(f32),
	Padding(f32),
	PaddingHorizontal(f32),
	PaddingLeft(f32),
	PaddingRight(f32),
	PaddingTop(f32),
	PaddingVertical(f32),
	Position(PositionType),
	Right(f32),
	Start(f32),
	Top(f32),
	Width(f32)
}

// Public re-exports of Yoga enums
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Align {
	Auto = 0,
	FlexStart = 1,
	Center = 2,
	FlexEnd = 3,
	Stretch = 4,
	Baseline = 5,
	SpaceBetween = 6,
	SpaceAround = 7
}

impl From<Align> for internal::YGAlign {
	fn from(a: Align) -> internal::YGAlign {
		match a {
			Align::Auto => internal::YGAlign::YGAlignAuto,
			Align::FlexStart => internal::YGAlign::YGAlignFlexStart,
			Align::Center => internal::YGAlign::YGAlignCenter,
			Align::FlexEnd => internal::YGAlign::YGAlignFlexEnd,
			Align::Stretch => internal::YGAlign::YGAlignStretch,
			Align::Baseline => internal::YGAlign::YGAlignBaseline,
			Align::SpaceBetween => internal::YGAlign::YGAlignSpaceBetween,
			Align::SpaceAround => internal::YGAlign::YGAlignSpaceAround
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Dimension {
	Width = 0,
	Height = 1
}

impl From<Dimension> for internal::YGDimension {
	fn from(d: Dimension) -> internal::YGDimension {
		match d {
			Dimension::Width => internal::YGDimension::YGDimensionWidth,
			Dimension::Height => internal::YGDimension::YGDimensionHeight
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
	Inherit = 0,
	LTR = 1,
	RTL = 2
}

impl From<Direction> for internal::YGDirection {
	fn from(d: Direction) -> internal::YGDirection {
		match d {
			Direction::Inherit => internal::YGDirection::YGDirectionInherit,
			Direction::LTR => internal::YGDirection::YGDirectionLTR,
			Direction::RTL => internal::YGDirection::YGDirectionRTL
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Display {
	Flex = 0,
	None = 1
}

impl From<Display> for internal::YGDisplay {
	fn from(d: Display) -> internal::YGDisplay {
		match d {
			Display::Flex => internal::YGDisplay::YGDisplayFlex,
			Display::None => internal::YGDisplay::YGDisplayNone
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Edge {
	Left = 0,
	Top = 1,
	Right = 2,
	Bottom = 3,
	Start = 4,
	End = 5,
	Horizontal = 6,
	Vertical = 7,
	All = 8
}

impl From<Edge> for internal::YGEdge {
	fn from(e: Edge) -> internal::YGEdge {
		match e {
			Edge::Left => internal::YGEdge::YGEdgeLeft,
			Edge::Top => internal::YGEdge::YGEdgeTop,
			Edge::Right => internal::YGEdge::YGEdgeRight,
			Edge::Bottom => internal::YGEdge::YGEdgeBottom,
			Edge::Start => internal::YGEdge::YGEdgeStart,
			Edge::End => internal::YGEdge::YGEdgeEnd,
			Edge::Horizontal => internal::YGEdge::YGEdgeHorizontal,
			Edge::Vertical => internal::YGEdge::YGEdgeVertical,
			Edge::All => internal::YGEdge::YGEdgeAll
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FlexDirection {
	Column = 0,
	ColumnReverse = 1,
	Row = 2,
	RowReverse = 3,
}

impl From<FlexDirection> for internal::YGFlexDirection {
	fn from(f: FlexDirection) -> internal::YGFlexDirection {
		match f {
			FlexDirection::Column => internal::YGFlexDirection::YGFlexDirectionColumn,
			FlexDirection::ColumnReverse => internal::YGFlexDirection::YGFlexDirectionColumnReverse,
			FlexDirection::Row => internal::YGFlexDirection::YGFlexDirectionRow,
			FlexDirection::RowReverse => internal::YGFlexDirection::YGFlexDirectionRowReverse
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Justify {
	FlexStart = 0,
	Center = 1,
	FlexEnd = 2,
	SpaceBetween = 3,
	SpaceAround = 4
}

impl From<Justify> for internal::YGJustify {
	fn from(j: Justify) -> internal::YGJustify {
		match j {
			Justify::FlexStart => internal::YGJustify::YGJustifyFlexStart,
			Justify::Center => internal::YGJustify::YGJustifyCenter,
			Justify::FlexEnd => internal::YGJustify::YGJustifyFlexEnd,
			Justify::SpaceBetween => internal::YGJustify::YGJustifySpaceBetween,
			Justify::SpaceAround => internal::YGJustify::YGJustifySpaceAround
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogLevel {
	Error = 0,
	Warn = 1,
	Info = 2,
	Debug = 3,
	Verbose = 4,
	Fatal = 5
}

impl From<LogLevel> for internal::YGLogLevel {
	fn from(l: LogLevel) -> internal::YGLogLevel {
		match l {
			LogLevel::Error => internal::YGLogLevel::YGLogLevelError,
			LogLevel::Warn => internal::YGLogLevel::YGLogLevelWarn,
			LogLevel::Info => internal::YGLogLevel::YGLogLevelInfo,
			LogLevel::Debug => internal::YGLogLevel::YGLogLevelDebug,
			LogLevel::Verbose => internal::YGLogLevel::YGLogLevelVerbose,
			LogLevel::Fatal => internal::YGLogLevel::YGLogLevelFatal
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MeasureMode {
	Undefined = 0,
	Exactly = 1,
	AtMost = 2
}

impl From<MeasureMode> for internal::YGMeasureMode {
	fn from(m: MeasureMode) -> internal::YGMeasureMode {
		match m {
			MeasureMode::Undefined => internal::YGMeasureMode::YGMeasureModeUndefined,
			MeasureMode::Exactly => internal::YGMeasureMode::YGMeasureModeExactly,
			MeasureMode::AtMost => internal::YGMeasureMode::YGMeasureModeAtMost
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
	Default = 0,
	Text = 1
}

impl From<NodeType> for internal::YGNodeType {
	fn from(n: NodeType) -> internal::YGNodeType {
		match n {
			NodeType::Default => internal::YGNodeType::YGNodeTypeDefault,
			NodeType::Text => internal::YGNodeType::YGNodeTypeText
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Overflow {
	Visible = 0,
	Hidden = 1,
	Scroll = 2
}

impl From<Overflow> for internal::YGOverflow {
	fn from(o: Overflow) -> internal::YGOverflow {
		match o {
			Overflow::Visible => internal::YGOverflow::YGOverflowVisible,
			Overflow::Hidden => internal::YGOverflow::YGOverflowHidden,
			Overflow::Scroll => internal::YGOverflow::YGOverflowScroll
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PositionType {
	Relative = 0,
	Absolute = 1
}

impl From<PositionType> for internal::YGPositionType {
	fn from(p: PositionType) -> internal::YGPositionType {
		match p {
			PositionType::Relative => internal::YGPositionType::YGPositionTypeRelative,
			PositionType::Absolute => internal::YGPositionType::YGPositionTypeAbsolute
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrintOptions {
	Layout = 1,
	Style = 2,
	Children = 4
}

impl From<PrintOptions> for internal::YGPrintOptions {
	fn from(p: PrintOptions) -> internal::YGPrintOptions {
		match p {
			PrintOptions::Layout => internal::YGPrintOptions::YGPrintOptionsLayout,
			PrintOptions::Style => internal::YGPrintOptions::YGPrintOptionsStyle,
			PrintOptions::Children => internal::YGPrintOptions::YGPrintOptionsChildren
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StyleUnit {
	Undefined = 0,
	Point = 1,
	Percent = 2,
	Auto = 3
}

impl From<StyleUnit> for internal::YGUnit {
	fn from(s: StyleUnit) -> internal::YGUnit {
		match s {
			StyleUnit::Undefined => internal::YGUnit::YGUnitUndefined,
			StyleUnit::Point => internal::YGUnit::YGUnitPoint,
			StyleUnit::Percent => internal::YGUnit::YGUnitPercent,
			StyleUnit::Auto => internal::YGUnit::YGUnitAuto
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Wrap {
	NoWrap = 0,
	Wrap = 1,
	WrapReverse = 2
}

impl From<Wrap> for internal::YGWrap {
	fn from(w: Wrap) -> internal::YGWrap {
		match w {
			Wrap::NoWrap => internal::YGWrap::YGWrapNoWrap,
			Wrap::Wrap => internal::YGWrap::YGWrapWrap,
			Wrap::WrapReverse => internal::YGWrap::YGWrapWrapReverse
		}
	}
}

pub use std::f32::NAN as Undefined;

// Custom Rust API

#[derive(Debug)]
pub struct Node {
	inner_node: internal::YGNodeRef,
	should_free: bool
}

#[derive(Debug)]
pub struct Layout {
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
	pub width: f32,
	pub height: f32
}

impl Node {
	pub fn new() -> Node {
		Node {
			inner_node: unsafe { internal::YGNodeNew() },
			should_free: true
		}
	}

	pub fn reset(&mut self) {
		unsafe {
			internal::YGNodeReset(self.inner_node);
		}
	}

	pub fn apply_styles(&mut self, styles: Vec<FlexStyle>) {
		use FlexStyle::*;

		for style in styles {
			match style {
				AlignItems(align) => self.set_align_items(align),
				AlignSelf(align) => self.set_align_self(align),
				BorderBottomWidth(w) => self.set_border(Edge::Bottom, w),
				BorderLeftWidth(w) => self.set_border(Edge::Left, w),
				BorderRightWidth(w) => self.set_border(Edge::Right, w),
				BorderTopWidth(w) => self.set_border(Edge::Top, w),
				BorderWidth(w) => self.set_border(Edge::All, w),
				Bottom(b) => self.set_position(Edge::Bottom, b),
				Flex(f) => self.set_flex(f),
				FlexDirection(flex_direction) => self.set_flex_direction(flex_direction),
				FlexWrap(wrap) => self.set_flex_wrap(wrap),
				Height(h) => self.set_height(h),
				JustifyContent(justify) => self.set_justify_content(justify),
				Left(l) => self.set_position(Edge::Left, l),
				Margin(m) => self.set_margin(Edge::All, m),
				MarginBottom(m) => self.set_margin(Edge::Bottom, m),
				MarginHorizontal(m) => self.set_margin(Edge::Horizontal, m),
				MarginLeft(m) => self.set_margin(Edge::Left, m),
				MarginRight(m) => self.set_margin(Edge::Right, m),
				MarginTop(m) => self.set_margin(Edge::Top, m),
				MarginVertical(m) => self.set_margin(Edge::Vertical, m),
				Padding(p) => self.set_padding(Edge::All, p),
				PaddingHorizontal(p) => self.set_padding(Edge::Horizontal, p),
				PaddingLeft(p) => self.set_padding(Edge::Left, p),
				PaddingRight(p) => self.set_padding(Edge::Right, p),
				PaddingTop(p) => self.set_padding(Edge::Top, p),
				PaddingVertical(p) => self.set_padding(Edge::Vertical, p),
				Position(position_type) => self.set_position_type(position_type),
				Right(r) => self.set_position(Edge::Right, r),
				Start(s) => self.set_position(Edge::Start, s),
				Top(t) => self.set_position(Edge::Top, t),
				Width(w) => self.set_width(w)
			}
		}
	}

	pub fn insert_child(&mut self, child: &mut Node, index: u32) {
		let mut child = child;
		child.should_free = false;

		unsafe {
			internal::YGNodeInsertChild(self.inner_node, child.inner_node, index);
		}
	}

	pub fn child_count(&self) -> u32 {
		unsafe {
			internal::YGNodeGetChildCount(self.inner_node)
		}
	}

	pub fn set_direction(&mut self, direction: Direction) {
		unsafe {
			internal::YGNodeStyleSetDirection(self.inner_node, internal::YGDirection::from(direction));
		}
	}

	pub fn set_flex_direction(&mut self, direction: FlexDirection) {
		unsafe {
			internal::YGNodeStyleSetFlexDirection(self.inner_node, internal::YGFlexDirection::from(direction));
		}
	}

	pub fn set_justify_content(&mut self, justify: Justify) {
		unsafe {
			internal::YGNodeStyleSetJustifyContent(self.inner_node, internal::YGJustify::from(justify));
		}
	}

	pub fn set_align_content(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignContent(self.inner_node, internal::YGAlign::from(align));
		}
	}

	pub fn set_align_items(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignItems(self.inner_node, internal::YGAlign::from(align));
		}
	}

	pub fn set_align_self(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignSelf(self.inner_node, internal::YGAlign::from(align));
		}
	}

	pub fn set_position_type(&mut self, position_type: PositionType) {
		unsafe {
			internal::YGNodeStyleSetPositionType(self.inner_node, internal::YGPositionType::from(position_type));
		}
	}

	pub fn set_position(&mut self, edge: Edge, position: f32) {
		unsafe {
			internal::YGNodeStyleSetPosition(self.inner_node, internal::YGEdge::from(edge), position);
		}
	}

	pub fn set_flex_wrap(&mut self, wrap: Wrap) {
		unsafe {
			internal::YGNodeStyleSetFlexWrap(self.inner_node, internal::YGWrap::from(wrap));
		}
	}

	pub fn set_overflow(&mut self, overflow: Overflow) {
		unsafe {
			internal::YGNodeStyleSetOverflow(self.inner_node, internal::YGOverflow::from(overflow));
		}
	}

	pub fn set_flex(&mut self, flex: f32) {
		unsafe {
			internal::YGNodeStyleSetFlex(self.inner_node, flex);
		}
	}

	pub fn set_flex_grow(&mut self, flex_grow: f32) {
		unsafe {
			internal::YGNodeStyleSetFlexGrow(self.inner_node, flex_grow);
		}
	}

	pub fn set_flex_shrink(&mut self, flex_shrink: f32) {
		unsafe {
			internal::YGNodeStyleSetFlexShrink(self.inner_node, flex_shrink);
		}
	}

	pub fn set_flex_basis(&mut self, flex_basis: f32) {
		unsafe {
			internal::YGNodeStyleSetFlexBasis(self.inner_node, flex_basis);
		}
	}

	pub fn set_edge_position(&mut self, edge: Edge, position: f32) {
		unsafe {
			internal::YGNodeStyleSetPosition(self.inner_node, internal::YGEdge::from(edge), position);
		}
	}

	pub fn set_margin(&mut self, edge: Edge, margin: f32) {
		unsafe {
			internal::YGNodeStyleSetMargin(self.inner_node, internal::YGEdge::from(edge), margin);
		}
	}

	pub fn set_padding(&mut self, edge: Edge, padding: f32) {
		unsafe {
			internal::YGNodeStyleSetPadding(self.inner_node, internal::YGEdge::from(edge), padding);
		}
	}

	pub fn set_border(&mut self, edge: Edge, border: f32) {
		unsafe {
			internal::YGNodeStyleSetBorder(self.inner_node, internal::YGEdge::from(edge), border);
		}
	}

	pub fn set_width(&mut self, width: f32) {
		unsafe {
			internal::YGNodeStyleSetWidth(self.inner_node, width);
		}
	}

	pub fn set_height(&mut self, height: f32) {
		unsafe {
			internal::YGNodeStyleSetHeight(self.inner_node, height);
		}
	}

	pub fn set_min_width(&mut self, min_width: f32) {
		unsafe {
			internal::YGNodeStyleSetMinWidth(self.inner_node, min_width);
		}
	}

	pub fn set_min_height(&mut self, min_height: f32) {
		unsafe {
			internal::YGNodeStyleSetMinHeight(self.inner_node, min_height);
		}
	}

	pub fn set_max_width(&mut self, max_width: f32) {
		unsafe {
			internal::YGNodeStyleSetMaxWidth(self.inner_node, max_width);
		}
	}

	pub fn set_max_height(&mut self, max_height: f32) {
		unsafe {
			internal::YGNodeStyleSetMaxHeight(self.inner_node, max_height);
		}
	}

	pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
		unsafe {
			internal::YGNodeStyleSetAspectRatio(self.inner_node, aspect_ratio);
		}
	}

	pub fn calculate_layout(&mut self, available_width: f32, available_height: f32, parent_direction: Direction) {
		unsafe {
			internal::YGNodeCalculateLayout(self.inner_node, available_width, available_height, internal::YGDirection::from(parent_direction));
		}
	}

	pub fn get_layout(&self) -> Layout {
		unsafe {
			Layout {
				left: internal::YGNodeLayoutGetLeft(self.inner_node),
				right: internal::YGNodeLayoutGetRight(self.inner_node),
				top: internal::YGNodeLayoutGetTop(self.inner_node),
				bottom: internal::YGNodeLayoutGetBottom(self.inner_node),
				width: internal::YGNodeLayoutGetWidth(self.inner_node),
				height: internal::YGNodeLayoutGetHeight(self.inner_node)
			}
		}
	}
}

impl Drop for Node {
	fn drop(&mut self) {
		if self.should_free {
			unsafe {
				internal::YGNodeFreeRecursive(self.inner_node);
			}
		}		
	}
}

#[test]
fn test_absolute_layout_width_height_start_top() {
	use FlexStyle::*;

	let mut root = Node::new();

	root.apply_styles(vec!(
		Width(100.0),
		Height(100.0)
	));

	let mut root_child_0 = Node::new();

	root_child_0.apply_styles(vec!(
		Position(PositionType::Absolute),
		Start(10.0),
		Top(10.0),
		Width(10.0),
		Height(10.0)
	));

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(10.0, child_layout.left);
	assert_eq!(10.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_layout.left);
	assert_eq!(10.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);
}
