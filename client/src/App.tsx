import React, { useEffect, useState } from "react";
import {
  AppBar,
  Container,
  CssBaseline,
  Grid,
  Toolbar,
  Typography,
  makeStyles,
} from "@material-ui/core";

import HostCard from "./HostCard";

const useStyles = makeStyles((theme) => ({
  root: {
    height: "100%",
  },
  title: {
    marginTop: theme.spacing(3),
    marginBottom: theme.spacing(3),
  },
  grid: {
    width: "100%",
  },
}));

interface Host {
  id: number;
  name: string;
  ip_address: string;
  mac_address: string;
}

async function fetchHosts(): Promise<Host[]> {
  const res = await fetch("/api/hosts");
  const data = await res.json();
  return data;
}

function App() {
  const classes = useStyles();
  const [hosts, setHosts] = useState<Host[]>();

  useEffect(() => {
    fetchHosts().then(setHosts);
  }, []);

  return (
    <div className={classes.root}>
      <CssBaseline />
      <AppBar position="sticky">
        <Toolbar>
          <Typography variant="h6">Vigil</Typography>
        </Toolbar>
      </AppBar>
      <Container>
        <div>
          <Typography variant="h2" className={classes.title}>
            My Hosts
          </Typography>
          <Grid container spacing={2} className={classes.grid}>
            {hosts &&
              hosts.map((host) => (
                <Grid key={host.id} item>
                  <HostCard
                    name={host.name}
                    ipAddress={host.ip_address}
                    macAddress={host.mac_address}
                  />
                </Grid>
              ))}
          </Grid>
        </div>
      </Container>
    </div>
  );
}

export default App;
