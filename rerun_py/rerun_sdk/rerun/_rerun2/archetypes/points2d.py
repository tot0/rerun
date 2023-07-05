# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from dataclasses import dataclass, field

from .._baseclasses import Archetype

__all__ = ["Points2D"]

from .. import components

## --- Points2D --- ##


@dataclass
class Points2D(Archetype):
    """
    A 2D point cloud with positions and optional colors, radii, labels, etc.

    Example
    -------
    ```python
    import rerun as rr

    rr.init("points", spawn=True)

    rr.log_any("simple", rr.Points2D([[0, 0], [1, 1]]))

    # Log an extra rect to set the view bounds
    rr.log_rect("bounds", [0, 0, 4, 3], rect_format=rr.RectFormat.XCYCWH)
    ```
    """

    points: components.Point2DArray = field(metadata={"component": "primary"})
    """
    All the actual 2D points that make up the point cloud.
    """

    radii: components.RadiusArray | None = field(default=None, metadata={"component": "secondary"})
    """
    Optional radii for the points, effectively turning them into circles.
    """

    colors: components.ColorArray | None = field(default=None, metadata={"component": "secondary"})
    """
    Optional colors for the points.

    The colors are interpreted as RGB or RGBA in sRGB gamma-space,
    As either 0-1 floats or 0-255 integers, with separate alpha.
    """

    labels: components.LabelArray | None = field(default=None, metadata={"component": "secondary"})
    """
    Optional text labels for the points.
    """

    draw_order: components.DrawOrderArray | None = field(default=None, metadata={"component": "secondary"})
    """
    An optional floating point value that specifies the 2D drawing order.
    Objects with higher values are drawn on top of those with lower values.

    The default for 2D points is 30.0.
    """

    class_ids: components.ClassIdArray | None = field(default=None, metadata={"component": "secondary"})
    """
    Optional class Ids for the points.

    The class ID provides colors and labels if not specified explicitly.
    """

    keypoint_ids: components.KeypointIdArray | None = field(default=None, metadata={"component": "secondary"})
    """
    Optional keypoint IDs for the points, identifying them within a class.

    If keypoint IDs are passed in but no class IDs were specified, the class ID will
    default to 0.
    This is useful to identify points within a single classification (which is identified
    with `class_id`).
    E.g. the classification might be 'Person' and the keypoints refer to joints on a
    detected skeleton.
    """

    instance_keys: components.InstanceKeyArray | None = field(default=None, metadata={"component": "secondary"})
    """
    Unique identifiers for each individual point in the batch.
    """

    def __str__(self) -> str:
        s = f"rr.{type(self).__name__}(\n"

        from dataclasses import fields

        for fld in fields(self):
            if "component" in fld.metadata:
                comp: components.Component = getattr(self, fld.name)
                if datatype := getattr(comp, "type"):
                    name = comp.extension_name
                    typ = datatype.storage_type
                    s += f"  {name}<{typ}>(\n    {comp.to_pylist()}\n  )\n"

        s += ")"

        return s

    def __repr__(self) -> str:
        return str(self)

    def __init__(
        self,
        points: components.Point2DArrayLike,
        *,
        radii: components.RadiusArrayLike | None = None,
        colors: components.ColorArrayLike | None = None,
        labels: components.LabelArrayLike | None = None,
        draw_order: components.DrawOrderLike | None = None,
        class_ids: components.ClassIdArrayLike | None = None,
        keypoint_ids: components.KeypointIdArrayLike | None = None,
        instance_keys: components.InstanceKeyArrayLike | None = None,
    ) -> None:
        # Required components
        self.points = components.Point2DArray.from_similar(points)

        # Optional components

        self.radii = components.RadiusArray.from_similar(radii)
        self.colors = components.ColorArray.from_similar(colors)
        self.labels = components.LabelArray.from_similar(labels)
        self.draw_order = components.DrawOrderArray.from_similar(draw_order)
        self.class_ids = components.ClassIdArray.from_similar(class_ids)
        self.keypoint_ids = components.KeypointIdArray.from_similar(keypoint_ids)
        self.instance_keys = components.InstanceKeyArray.from_similar(instance_keys)
