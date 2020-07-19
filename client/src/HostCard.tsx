import React from "react";
import {
  Button,
  Card,
  CardActions,
  CardContent,
  CardMedia,
  Typography,
  makeStyles,
} from "@material-ui/core";

import pc from "./assets/pc-display.jpg";

const useStyles = makeStyles((theme) => ({
  root: {
    minWidth: 375,
  },
  media: {
    paddingTop: "65%",
  },
}));

interface Props {
  name: string;
  ipAddress: string;
  macAddress: string;
}

function HostCard({ name, ipAddress, macAddress }: Props) {
  const classes = useStyles();
  return (
    <Card className={classes.root}>
      <CardMedia image={pc} className={classes.media} />
      <CardContent>
        <Typography gutterBottom variant="h5">
          {name}
        </Typography>
        <Typography variant="body2" color="textSecondary">
          {ipAddress}
        </Typography>
        <Typography variant="body2" color="textSecondary">
          {macAddress}
        </Typography>
      </CardContent>
      <CardActions>
        <Button color="primary">Wake</Button>
      </CardActions>
    </Card>
  );
}

export default HostCard;
