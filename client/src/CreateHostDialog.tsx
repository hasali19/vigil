import React, { useCallback, useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogTitle,
  TextField,
  DialogActions,
  Button,
  DialogContentText,
  makeStyles,
} from "@material-ui/core";

const useStyles = makeStyles({
  cancel: {
    color: "red",
  },
});

export interface Props {
  open: boolean;
  onSave?: (name: string, ipAddress: string, macAddress: string) => void;
  onCancel?: () => void;
}

function CreateHostDialog({ open, onSave, onCancel }: Props) {
  const classes = useStyles();

  const [name, setName] = useState("");
  const [ipAddress, setIpAddress] = useState("");
  const [macAddress, setMacAddress] = useState("");

  const [errors, setErrors] = useState({
    name: false,
    ipAddress: false,
    macAddress: false,
  });

  const clearErrors = useCallback(
    () => setErrors({ name: false, ipAddress: false, macAddress: false }),
    []
  );

  const save = useCallback(() => {
    let errors = false;

    clearErrors();

    if (/^\s*$/.test(name)) {
      setErrors((errors) => ({ ...errors, name: true }));
      errors = true;
    }

    // TODO: Proper validation for ip address and mac address

    if (/^\s*$/.test(ipAddress)) {
      setErrors((errors) => ({ ...errors, ipAddress: true }));
      errors = true;
    }

    if (/^\s*$/.test(macAddress)) {
      setErrors((errors) => ({ ...errors, macAddress: true }));
      errors = true;
    }

    if (onSave && !errors) {
      onSave(name, ipAddress, macAddress);
    }
  }, [name, ipAddress, macAddress, onSave, clearErrors]);

  const cancel = useCallback(() => {
    clearErrors();
    if (onCancel) {
      onCancel();
    }
  }, [onCancel, clearErrors]);

  return (
    <Dialog open={open} onClose={cancel} fullWidth>
      <DialogTitle>New host</DialogTitle>
      <DialogContent>
        <DialogContentText>Create a new host</DialogContentText>
        <TextField
          autoFocus
          label="Name"
          type="text"
          margin="dense"
          variant="filled"
          fullWidth
          value={name}
          error={errors.name}
          onChange={(e) => setName(e.target.value)}
        />
        <TextField
          label="IP Address"
          type="text"
          margin="dense"
          variant="filled"
          fullWidth
          value={ipAddress}
          error={errors.ipAddress}
          onChange={(e) => setIpAddress(e.target.value)}
        />
        <TextField
          label="MAC Address"
          type="text"
          margin="dense"
          variant="filled"
          fullWidth
          value={macAddress}
          error={errors.macAddress}
          onChange={(e) => setMacAddress(e.target.value)}
        />
      </DialogContent>
      <DialogActions>
        <Button color="primary" onClick={save}>
          Save
        </Button>
        <Button className={classes.cancel} onClick={cancel}>
          Cancel
        </Button>
      </DialogActions>
    </Dialog>
  );
}

export default CreateHostDialog;
